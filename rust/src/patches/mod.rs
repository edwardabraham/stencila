use crate::{
    errors::{invalid_patch_operation, invalid_patch_value},
    kernels::KernelSpace,
    methods::{compile::execute, encode::encode},
    utils::schemas,
};
use defaults::Defaults;
use derive_more::{Constructor, Deref, DerefMut};
use eyre::{bail, Result};
use inflector::cases::camelcase::to_camel_case;
use itertools::Itertools;
use prelude::{invalid_address, unpointable_type};
use schemars::{gen::SchemaGenerator, schema::Schema, JsonSchema};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_with::skip_serializing_none;
use similar::TextDiff;
use std::{
    any::{type_name, Any},
    collections::VecDeque,
    fmt::{self, Debug, Display},
    hash::Hasher,
    iter::FromIterator,
};
use stencila_schema::{
    Array, BlockContent, Boolean, InlineContent, Integer, ListItem, Node, Null, Number, Object,
};
use strum::{AsRefStr, ToString};

/// Are two nodes are the same type and value?
pub fn same<Type1, Type2>(node1: &Type1, node2: &Type2) -> bool
where
    Type1: Patchable,
    Type2: Clone + Send + 'static,
{
    node1.is_same(node2).is_ok()
}

/// Do two nodes of the same type have equal value?
pub fn equal<Type>(node1: &Type, node2: &Type) -> bool
where
    Type: Patchable,
{
    node1.is_equal(node2).is_ok()
}

/// Generate a [`Patch`] describing the difference between two nodes of the same type.
pub fn diff<Type>(node1: &Type, node2: &Type) -> Patch
where
    Type: Patchable,
{
    let mut differ = Differ::default();
    node1.diff_same(&mut differ, node2);
    Patch::new(differ.ops)
}

/// Display the difference between two nodes as a "unified diff" of the nodes
/// converted to a given format.
///
/// This can provide a more intuitive way of visualizing the differences between the
/// nodes than the raw [`Operation`]s. Note that this is slightly different from first
/// converting each node and then taking the diff in that this generates and applies a
/// patch. This means any change operations not generated or applied by the functions
/// in this module will not appear in the difference.
pub async fn diff_display(node1: &Node, node2: &Node, format: &str) -> Result<String> {
    let patch = diff(node1, node2);
    let patched = apply_new(node1, &patch)?;

    let old = encode(node1, "string://", format, None).await?;
    let new = encode(&patched, "string://", format, None).await?;

    let mut bytes = Vec::new();
    TextDiff::from_lines(&old, &new)
        .unified_diff()
        .to_writer(&mut bytes)
        .unwrap();

    let display = String::from_utf8(bytes)?;
    Ok(display)
}

/// Apply a [`Patch`] to a node.
pub fn apply<Type>(node: &mut Type, patch: &Patch) -> Result<()>
where
    Type: Patchable,
{
    node.apply_patch(patch)
}

/// Apply a [`Patch`] to a clone of a node.
///
/// In contrast to `apply`, this does not alter the original node.
pub fn apply_new<Type>(node: &Type, patch: &Patch) -> Result<Type>
where
    Type: Patchable + Clone,
{
    let mut node = node.clone();
    node.apply_patch(patch)?;
    Ok(node)
}

/// Merge changes from two or more derived versions of a node into
/// their common ancestor version.
///
/// This is equivalent to `git merge` except that there can be
/// more than two derived versions and conflicts are always resolved.
/// Conflicts are resolved by preferring the changes in 'later' derived
/// version (i.e. those that are later in the `derived` list).
///
/// # Arguments
///
/// - `ancestor`: The ancestor node
/// - `derived`: A list of derived nodes in ascending order of priority
///              when resolving merge conflicts i.e. the last in the list
///              will win over all other nodes that it conflicts with
pub fn merge<Type>(ancestor: &mut Type, derived: &[&Type]) -> Result<()>
where
    Type: Patchable,
{
    let patches: Vec<Patch> = derived.iter().map(|node| diff(ancestor, *node)).collect();

    // TODO transform operations (shift address based on other operations) and resolve conflicts
    tracing::warn!("Merging is work in progress");

    for patch in patches {
        apply(ancestor, &patch)?;
    }
    Ok(())
}

/// Resolve a child node from within a node using an address or id.
///
/// Intended to be able to fallback to using `id` if address can not be resolved
/// or resolves to a node with the incorrect `id`. However, borrow checker is not
/// making that possible yet.
pub fn resolve<Type>(
    node: &mut Type,
    address: Option<Address>,
    node_id: Option<String>,
) -> Result<Pointer>
where
    Type: Patchable,
{
    if let Some(mut address) = address {
        let pointer = node.resolve(&mut address)?;
        match pointer {
            Pointer::None => {
                bail!("Unable to resolve address `{}`", address.to_string())
                // TODO Do not bail, just warn and then find
            }
            _ => {
                // TODO check pointer id is consistent with node_id if supplied
                Ok(pointer)
            }
        }
    } else if let Some(node_id) = node_id {
        let pointer = node.find(&node_id);
        match pointer {
            Pointer::None => {
                bail!("Unable to find node with id `{}`", node_id)
            }
            _ => Ok(pointer),
        }
    } else {
        bail!("One of address or node id must be supplied to resolve a node")
    }
}

/// A slot, used as part of an [`Address`], to locate a value within a `Node` tree.
///
/// Slots can be used to identify a part of a larger object.
///
/// The `Name` variant can be used to identify:
///
/// - the property name of a `struct`
/// - the key of a `HashMap<String, ...>`
///
/// The `Integer` variant can be used to identify:
///
/// - the index of a `Vec`
/// - the index of a Unicode character in a `String`
///
/// The `None` variant is used in places where a `Slot` is required
/// but none applies to the particular type or use case.
///
/// In contrast to JSON Patch, which uses a [JSON Pointer](http://tools.ietf.org/html/rfc6901)
/// to describe the location of additions and removals, slots offer improved performance and
/// type safety.
#[derive(Debug, Clone, JsonSchema, Serialize, Deserialize, AsRefStr)]
#[serde(untagged)]
#[schemars(deny_unknown_fields)]
pub enum Slot {
    Index(usize),
    Name(String),
}

impl ToString for Slot {
    fn to_string(&self) -> String {
        match self {
            Slot::Name(name) => name.clone(),
            Slot::Index(index) => index.to_string(),
        }
    }
}

/// The address, defined by a list of [`Slot`]s, of a value within `Node` tree.
///
/// Implemented as a double-ended queue. Given that addresses usually have less than
/// six slots it may be more performant to use a stack allocated `tinyvec` here instead.
///
/// Note: This could instead have be called a "Path", but that name was avoided because
/// of potential confusion with file system paths.
#[derive(
    Debug, Clone, Default, Constructor, Deref, DerefMut, JsonSchema, Serialize, Deserialize,
)]
#[schemars(deny_unknown_fields)]
pub struct Address(VecDeque<Slot>);

impl Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let me = self
            .iter()
            .map(|slot| slot.to_string())
            .collect_vec()
            .join(".");
        write!(f, "{}", me)
    }
}

impl Address {
    /// Create an empty address
    pub fn empty() -> Self {
        Self::default()
    }

    /// Concatenate an address with another
    fn concat(&self, other: &Address) -> Self {
        let mut concat = self.clone();
        concat.append(&mut other.clone());
        concat
    }

    /// Creates a DOM compatible address by ensuring an name slots
    /// are camelCased (the convention used in Stencila Schema and thus
    /// in `itemprop` attributes) rather than snake_cased (as in Rust).
    fn to_dom_address(&self) -> Self {
        Address::new(
            self.iter()
                .map(|slot| match slot {
                    Slot::Index(..) => slot.clone(),
                    Slot::Name(name) => Slot::Name(to_camel_case(name)),
                })
                .collect(),
        )
    }
}

impl From<usize> for Address {
    fn from(index: usize) -> Address {
        Address(VecDeque::from_iter([Slot::Index(index)]))
    }
}

impl From<&str> for Address {
    fn from(name: &str) -> Address {
        Address(VecDeque::from_iter([Slot::Name(name.to_string())]))
    }
}

#[derive(Debug)]
pub enum Pointer<'lt> {
    None,
    Some,
    Inline(&'lt mut InlineContent),
    Block(&'lt mut BlockContent),
    Node(&'lt mut Node),
}

impl<'lt> Pointer<'lt> {
    /// Apply a patch to the node that is pointed to
    pub fn patch(&mut self, patch: &Patch) -> Result<()> {
        match self {
            Pointer::Inline(node) => node.apply_patch(patch),
            Pointer::Block(node) => node.apply_patch(patch),
            Pointer::Node(node) => node.apply_patch(patch),
            _ => bail!("Invalid node pointer: {:?}", self),
        }
    }

    /// Execute the node that is pointed to
    ///
    /// Returns a patch representing the change in the node resulting from
    /// the execution (usually to it's outputs)
    pub fn execute(&mut self, kernels: &mut KernelSpace) -> Result<Patch> {
        let patch = match self {
            Pointer::Inline(node) => {
                // TODO: Reinstate real diffing, rather than wholesale replacement
                //let pre = node.clone();
                execute(*node, kernels)?;
                //diff(&pre, node)
                Patch::new(vec![Operation::Replace {
                    address: Address::empty(),
                    items: 1,
                    value: Box::new(node.clone()),
                    length: 1,
                }])
            }
            Pointer::Block(node) => {
                // TODO: Reinstate real diffing, rather than wholesale replacement
                //let pre = node.clone();
                execute(*node, kernels)?;
                //diff(&pre, node)
                Patch::new(vec![Operation::Replace {
                    address: Address::empty(),
                    items: 1,
                    value: Box::new(node.clone()),
                    length: 1,
                }])
            }
            Pointer::Node(node) => {
                let pre = node.clone();
                execute(*node, kernels)?;
                diff(&pre, node)
            }
            _ => bail!("Invalid node pointer: {:?}", self),
        };
        Ok(patch)
    }
}

/// Type for the `value` property of `Add` and `Replace` operations
///
/// This open, dynamic type could be replaced with a enum (with a fixed number
/// of type variants) but that would require substantial refactoring
pub type Value = Box<dyn Any + Send>;

/// The operations that can be used in a patch to mutate one node into another.
///
/// These are the same operations as described in [JSON Patch](http://jsonpatch.com/)
/// (with the exception of `copy` and `test`). Note that `Replace` and `Move` could be
/// represented by combinations of `Remove` and `Add`. They are included as a means of
/// providing more semantically meaningful patches, and more space efficient serializations
/// (e.g. it is not necessary to represent the value being moved or copied).
///
/// In addition, there is a `Transform` operation which can be used describe the transformation
/// of a node to another type, having a similar structure. Examples includes:
///
/// - a `String` to an `Emphasis`
/// - a `Paragraph` to a `QuoteBlock`
/// - a `CodeChunk` to a `CodeBlock`
///
/// The `length` field on `Add` and `Replace` is not necessary for applying operations, but
/// is useful for generating them and for determining if there are conflicts between two patches
/// without having to downcast the `value`.
///
/// Note that for `String`s the integers in `address`, `items` and `length` all refer to Unicode
/// characters not bytes.
#[derive(Debug, JsonSchema, Serialize, Deserialize, ToString)]
#[serde(tag = "type")]
#[schemars(deny_unknown_fields)]
pub enum Operation {
    /// Add a value
    #[schemars(title = "OperationAdd")]
    Add {
        /// The address to which to add the value
        address: Address,

        /// The value to add
        #[serde(
            serialize_with = "Operation::value_serialize",
            deserialize_with = "Operation::value_deserialize"
        )]
        #[schemars(schema_with = "Operation::value_schema")]
        value: Value,

        /// The number of items added
        length: usize,
    },
    /// Remove one or more values
    #[schemars(title = "OperationRemove")]
    Remove {
        /// The address from which to remove the value(s)
        address: Address,

        /// The number of items to remove
        items: usize,
    },
    /// Replace one or more values
    #[schemars(title = "OperationReplace")]
    Replace {
        /// The address which should be replaced
        address: Address,

        /// The number of items to replace
        items: usize,

        /// The replacement value
        #[serde(
            serialize_with = "Operation::value_serialize",
            deserialize_with = "Operation::value_deserialize"
        )]
        #[schemars(schema_with = "Operation::value_schema")]
        value: Value,

        /// The number of items added
        length: usize,
    },
    /// Move a value from one address to another
    #[schemars(title = "OperationMove")]
    Move {
        /// The address from which to remove the value
        from: Address,

        /// The number of items to move
        items: usize,

        /// The address to which to add the items
        to: Address,
    },
    /// Transform a value from one type to another
    #[schemars(title = "OperationTransform")]
    Transform {
        /// The address of the `Node` to transform
        address: Address,

        /// The type of `Node` to transform from
        from: String,

        /// The type of `Node` to transform to
        to: String,
    },
}

impl Operation {
    /// Serialize the `value` field of an operation
    ///
    /// This is mainly for debugging and testing. Serialization of types is added as
    /// needed.
    fn value_serialize<S>(value: &Value, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Most value types just get serialized as normal
        macro_rules! serialize {
            ($type:ty) => {
                if let Some(value) = value.downcast_ref::<$type>() {
                    return value.serialize(serializer);
                }
                if let Some(value) = value.downcast_ref::<Vec<$type>>() {
                    return value.serialize(serializer);
                }
            };
            ($($type:ty)*) => {
                $(serialize!($type);)*
            }
        }
        serialize!(
            u8
            i32
            f32
            Boolean
            Integer
            Number
            String
            InlineContent
            BlockContent
            serde_json::Value
        );

        // For debugging purposes, other types get printed as their type name
        macro_rules! typename {
            ($type:ty) => {
                if value.downcast_ref::<$type>().is_some() {
                    return serializer.serialize_str(stringify!($type));
                }
            };
            ($($type:ty)*) => {
                $(typename!($type);)*
            }
        }
        typename!(
            Option<Boolean>
            Option<Integer>
            Option<Number>
            Option<String>
            Option<Node>
            Box<Boolean>
            Box<Integer>
            Box<Number>
            Box<String>
            Box<Node>
            Option<Box<Boolean>>
            Option<Box<Integer>>
            Option<Box<Number>>
            Option<Box<String>>
            Option<Box<Node>>
        );

        // Fallback to messaged that unserialized
        serializer.serialize_str("<unserialized type>")
    }

    /// Deserialize the `value` field of an operation
    ///
    /// This is needed so that the server can receive a `Patch` from the client and
    /// deserialize the JSON value into a `Value`.
    fn value_deserialize<'de, D>(deserializer: D) -> Result<Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        Ok(Box::new(value))
    }

    /// Generate the JSON Schema for the `value` property
    fn value_schema(_generator: &mut SchemaGenerator) -> Schema {
        schemas::typescript("any", true)
    }
}

/// A set of [`Operation`]s
#[skip_serializing_none]
#[derive(Debug, Default, JsonSchema, Serialize, Deserialize)]
#[schemars(deny_unknown_fields)]
pub struct Patch {
    /// The [`Operation`]s to apply
    ops: Vec<Operation>,

    /// The id of the node to which to apply this patch
    target: Option<String>,

    /// The id of the actor that generated this patch
    /// e.g. a web browser client, or file watcher
    actor: Option<String>,
}

impl Patch {
    fn new(ops: Vec<Operation>) -> Self {
        Patch {
            ops,
            target: None,
            actor: None,
        }
    }
}

/// A DOM operation used to mutate the DOM.
///
/// A `DomOperation` is the DOM version of an [`Operation`].
/// The same names for operation variants and their properties
/// are used with the following exception:
///
/// - the `value` property of `Add` and `Replace` is replaced by `html`, a HTML string
///   representing the node (usually a HTML `Element` or `Text` node), and `json`, a JSON
///   representation of the node (used for updating WebComponents).
///
/// - the `length` property of `Add` and `Replace` is not included because it is not
///   needed (for merge conflict resolution as it is in `Operation`).
#[derive(Debug, JsonSchema, Serialize)]
#[serde(tag = "type")]
#[schemars(deny_unknown_fields)]
pub enum DomOperation {
    /// Add one or more DOM nodes
    #[schemars(title = "DomOperationAdd")]
    Add {
        /// The address to which to add the DOM node(s)
        address: Address,

        /// The HTML to add
        html: String,

        /// The JSON value to add
        json: serde_json::Value,
    },
    /// Remove one or more DOM nodes
    #[schemars(title = "DomOperationRemove")]
    Remove {
        /// The address from which to remove the DOM node(s)
        address: Address,

        /// The number of items to remove
        items: usize,
    },
    /// Replace one or more DOM nodes
    #[schemars(title = "DomOperationReplace")]
    Replace {
        /// The address which should be replaced
        address: Address,

        /// The number of items to replace
        items: usize,

        /// The replacement HTML
        html: String,

        /// The JSON value to replace
        json: serde_json::Value,
    },
    /// Move a DOM node from one address to another
    #[schemars(title = "DomOperationMove")]
    Move {
        /// The address from which to remove the DOM node
        from: Address,

        /// The number of items to move
        items: usize,

        /// The address to which to add the items
        to: Address,
    },
    /// Transform a DOM node from one type to another
    #[schemars(title = "DomOperationTransform")]
    Transform {
        /// The address of the DOM node to transform
        address: Address,

        /// The type of `Node` to transform from
        from: String,

        /// The type of `Node` to transform to
        to: String,
    },
}

impl DomOperation {
    /// Create a `DomOperation` from an `Operation`
    fn new(op: &Operation) -> DomOperation {
        match op {
            Operation::Add { address, value, .. } => DomOperation::Add {
                address: address.to_dom_address(),
                html: DomOperation::value_html(address, value),
                json: DomOperation::value_json(value),
            },

            Operation::Remove { address, items, .. } => DomOperation::Remove {
                address: address.to_dom_address(),
                items: *items,
            },

            Operation::Replace {
                address,
                items,
                value,
                ..
            } => DomOperation::Replace {
                address: address.to_dom_address(),
                items: *items,
                html: DomOperation::value_html(address, value),
                json: DomOperation::value_json(value),
            },

            Operation::Move { from, items, to } => DomOperation::Move {
                from: from.to_dom_address(),
                items: *items,
                to: to.to_dom_address(),
            },

            Operation::Transform { address, from, to } => DomOperation::Transform {
                address: address.to_dom_address(),
                from: from.clone(),
                to: to.clone(),
            },
        }
    }

    /// Generate HTML for the `value` field of an operation
    fn value_html(address: &Address, value: &Value) -> String {
        use crate::methods::encode::html::{Context, ToHtml};

        let slot = address.back();
        let context = Context::new();

        // Convert a node, boxed node, or vector of nodes to HTML
        macro_rules! to_html {
            ($type:ty) => {
                if let Some(node) = value.downcast_ref::<$type>() {
                    return node.to_html(
                        &slot.map(|slot| slot.to_string()).unwrap_or_default(),
                        &context
                    )
                }
                if let Some(boxed) = value.downcast_ref::<Box<$type>>() {
                    return boxed.to_html(
                        &slot.map(|slot| slot.to_string()).unwrap_or_default(),
                        &context
                    )
                }
                if let Some(nodes) = value.downcast_ref::<Vec<$type>>() {
                    return match slot {
                        // If the slot is a name then we're adding or replacing a property so we
                        // want the `Vec` to have a wrapper element with the name as the slot attribute
                        Some(Slot::Name(name)) => nodes.to_html(name, &context),
                        // If the slot is an index then we're adding or replacing items in a
                        // vector so we don't want a wrapper element
                        Some(Slot::Index(..)) | None => nodes.to_html("", &context),
                    };
                }
            };
            ($($type:ty)*) => {
                $(to_html!($type);)*
            }
        }

        // Types roughly ordered by expected incidence (more commonly used types in
        // patches first)
        to_html!(
            InlineContent
            BlockContent
            Node

            String
            Number
            Integer
            Boolean
            Array
            Object
            Null
        );

        // The value may be a JSON value (if this patch was sent from a client)
        // In that case we want to deserialize it to one of the above types and
        // then encode as HTML
        if let Some(value) = value.downcast_ref::<serde_json::Value>() {
            if let Some(str) = value.as_str() {
                return str.to_string();
            } else if let Ok(nodes) = serde_json::from_value::<InlineContent>(value.clone()) {
                return nodes.to_html("", &context);
            } else if let Ok(nodes) = serde_json::from_value::<Vec<InlineContent>>(value.clone()) {
                return nodes.to_html("", &context);
            } else if let Ok(nodes) = serde_json::from_value::<BlockContent>(value.clone()) {
                return nodes.to_html("", &context);
            } else if let Ok(nodes) = serde_json::from_value::<Vec<BlockContent>>(value.clone()) {
                return nodes.to_html("", &context);
            } else if let Ok(nodes) = serde_json::from_value::<ListItem>(value.clone()) {
                return nodes.to_html("", &context);
            } else if let Ok(nodes) = serde_json::from_value::<Vec<ListItem>>(value.clone()) {
                return nodes.to_html("", &context);
            } else {
                tracing::error!(
                    "Unhandled JSON value type when generating HTML for `DomOperation`"
                );
                return ["<span class=\"todo\">", &value.to_string(), "</span>"].concat();
            }
        }

        tracing::error!("Unhandled value type when generating HTML for `DomOperation`");
        "<span class=\"todo\">TODO</span>".to_string()
    }

    /// Generate JSON for the `value` field of an operation
    fn value_json(value: &Value) -> serde_json::Value {
        // Convert a node, boxed node, or vector of nodes to HTML
        macro_rules! to_json {
            ($type:ty) => {
                if let Some(node) = value.downcast_ref::<$type>() {
                    return serde_json::to_value(node).expect("Should convert to JSON")
                }
                if let Some(boxed) = value.downcast_ref::<Box<$type>>() {
                    return serde_json::to_value(boxed).expect("Should convert to JSON")
                }
                if let Some(nodes) = value.downcast_ref::<Vec<$type>>() {
                    return serde_json::to_value(nodes).expect("Should convert to JSON")
                }
            };
            ($($type:ty)*) => {
                $(to_json!($type);)*
            }
        }

        // As above, types roughly ordered by expected incidence
        to_json!(
            InlineContent
            BlockContent
            Node

            String
            Number
            Integer
            Boolean
            Array
            Object
            Null
        );

        // The value may be a JSON value (if this patch was sent from a client)
        if let Some(value) = value.downcast_ref::<serde_json::Value>() {
            return value.clone();
        }

        tracing::error!("Unhandled value type when generating JSON for `DomOperation`");
        serde_json::Value::String("TODO".to_string())
    }
}

/// A set of [`DomOperation`]s to be applied to some DOM element
#[skip_serializing_none]
#[derive(Debug, JsonSchema, Serialize)]
#[schemars(deny_unknown_fields)]
pub struct DomPatch {
    /// The [`DomOperation`]s to apply
    ops: Vec<DomOperation>,

    /// The id of the node to which to apply this patch
    pub target: Option<String>,

    /// The id of the actor that generated this patch
    /// e.g. a web browser client, or file watcher
    actor: Option<String>,
}

impl DomPatch {
    pub fn new(patch: &Patch) -> DomPatch {
        DomPatch {
            ops: patch.ops.iter().map(|op| DomOperation::new(op)).collect(),
            target: patch.target.clone(),
            actor: patch.actor.clone(),
        }
    }
}

/// A differencing `struct` used as an optimization to track the address describing the
/// current location in a node tree while walking over it.
#[derive(Defaults)]
pub struct Differ {
    /// The list of address describing the current location in a node tree.
    address: Address,

    /// The operations generated by walking over a node tree.
    ops: Vec<Operation>,
}

impl Differ {
    /// Difference a field of a `struct` or an item of a `HashMap`.
    ///
    /// Adds a `Name` key to `address` and then differences the two values.
    pub fn field<Type: Patchable>(&mut self, name: &str, value1: &Type, value2: &Type) {
        self.address.push_back(Slot::Name(name.to_string()));
        value1.diff_same(self, value2);
        self.address.pop_back();
    }

    /// Difference an item in a `Vec`.
    ///
    /// Adds an `Index` key to `address` and then differences the two values.
    pub fn item<Type: Patchable>(&mut self, index: usize, value1: &Type, value2: &Type) {
        self.address.push_back(Slot::Index(index));
        value1.diff_same(self, value2);
        self.address.pop_back();
    }

    /// Append a list of operations nested within the current address
    pub fn append(&mut self, ops: Vec<Operation>) {
        for op in ops {
            let op = match op {
                Operation::Add {
                    address,
                    value,
                    length,
                } => Operation::Add {
                    address: self.address.concat(&address),
                    value,
                    length,
                },
                Operation::Remove { address, items } => Operation::Remove {
                    address: self.address.concat(&address),
                    items,
                },
                Operation::Replace {
                    address,
                    items,
                    value,
                    length,
                } => Operation::Replace {
                    address: self.address.concat(&address),
                    items,
                    value,
                    length,
                },
                Operation::Move { from, items, to } => Operation::Move {
                    from: self.address.concat(&from),
                    items,
                    to: self.address.concat(&to),
                },
                Operation::Transform { address, from, to } => Operation::Transform {
                    address: self.address.concat(&address),
                    from,
                    to,
                },
            };
            self.ops.push(op)
        }
    }

    /// Add an `Add` operation to the patch.
    pub fn add<Value: Clone + Send + 'static>(&mut self, value: &Value) {
        self.ops.push(Operation::Add {
            address: self.address.clone(),
            value: Box::new(value.clone()),
            length: 1,
        })
    }

    /// Add a `Remove` operation to the patch.
    pub fn remove(&mut self) {
        self.ops.push(Operation::Remove {
            address: self.address.clone(),
            items: 1,
        })
    }

    /// Add a `Replace` operation to the patch.
    pub fn replace<Value: Clone + Send + 'static>(&mut self, value: &Value) {
        self.ops.push(Operation::Replace {
            address: self.address.clone(),
            items: 1,
            value: Box::new(value.clone()),
            length: 1,
        })
    }

    /// Add a `Transform` operation to the patch.
    pub fn transform(&mut self, from: &str, to: &str) {
        self.ops.push(Operation::Transform {
            address: self.address.clone(),
            from: from.into(),
            to: to.into(),
        })
    }
}

pub trait Patchable {
    /// Resolve an [`Address`] into a node [`Pointer`].
    ///
    /// If the address in empty, and the node is represented in one of the variants of [`Pointer`]
    /// (at the time of writing `Node`, `BlockContent` and `InlineContent`), then it should return
    /// a pointer to itself. Otherwise it should return an "unpointable" type error.
    ///
    /// If the address is not empty then it should be passed on to any child nodes.
    ///
    /// If the address is invalid for the type (e.g. a non-empty address for a leaf node, a name
    /// slot used for a vector) then implementations should return an error.
    ///
    /// The default implementation is only suitable for leaf nodes that are not pointable.
    fn resolve(&mut self, address: &mut Address) -> Result<Pointer> {
        match address.is_empty() {
            true => bail!(unpointable_type::<Self>(address)),
            false => bail!(invalid_address::<Self>(address)),
        }
    }

    /// Find a node based on its `id` and return a [`Pointer`] to it.
    ///
    /// This is less efficient than `resolve` (given that it must visit all nodes until one is
    /// found with a matching id). However, it may be necessary to use when an [`Address`] is not available.
    ///
    /// If the node has a matching `id` property then it should return `Pointer::Some` which indicates
    /// that the `id` is matched . This allows the parent type e.g `InlineContent` to populate the
    /// "useable" pointer variants e.g. `Pointer::InlineContent`.
    ///
    /// Otherwise, if the node has children it should call `find` on them and return `Pointer::None` if
    /// no children have a matching `id`.
    ///
    /// The default implementation is only suitable for leaf nodes that do not have an `id` property.
    fn find(&mut self, _id: &str) -> Pointer {
        Pointer::None
    }

    /// Test whether a node is the same as (i.e. equal type and equal value)
    /// another node of any type.
    fn is_same<Other: Any + Clone + Send>(&self, other: &Other) -> Result<()>;

    /// Test whether a node is equal to (i.e. equal value) a node of the same type.
    fn is_equal(&self, other: &Self) -> Result<()>;

    /// Generate a hash of the patchable content of a node
    ///
    /// Used for identifying unique values, particularly when diffing sequences.
    fn make_hash<H: Hasher>(&self, state: &mut H);

    /// Generate the operations needed to mutate this node so that is the same as
    /// a node of any other type.
    ///
    /// `Other` needs to be `Clone` so that if necessary, we can keep a copy of it in a
    /// `Add` or `Replace operation.
    fn diff<Other: Any + Clone + Send>(&self, differ: &mut Differ, other: &Other);

    /// Generate the operations needed to mutate this node so that it is equal
    /// to a node of the same type.
    fn diff_same(&self, differ: &mut Differ, other: &Self);

    /// Generate the operations needed to mutate this node so that it is the
    /// same as a node of any other type.
    ///
    /// This allows node types to define a `Transform` patch operation, which
    /// is more semantically explicit, and will usually require less data changes
    /// than a full `Replace` operation. An example is transforming a `Emphasis`
    /// node to a `Strong` node.
    ///
    /// The default implementation simply replaces the current node. Override as
    /// suits.
    fn diff_other<Other: Any + Clone + Send>(&self, differ: &mut Differ, other: &Other) {
        differ.replace(other)
    }

    /// Apply a patch to this node.
    fn apply_patch(&mut self, patch: &Patch) -> Result<()> {
        tracing::debug!("Applying patch to type '{}'", type_name::<Self>());
        for op in &patch.ops {
            self.apply_op(op)?
        }
        Ok(())
    }

    /// Apply an operation to this node.
    fn apply_op(&mut self, op: &Operation) -> Result<()> {
        match op {
            Operation::Add { address, value, .. } => self.apply_add(&mut address.clone(), value),
            Operation::Remove { address, items } => self.apply_remove(&mut address.clone(), *items),
            Operation::Replace {
                address,
                items,
                value,
                ..
            } => self.apply_replace(&mut address.clone(), *items, value),
            Operation::Move { from, items, to } => {
                self.apply_move(&mut from.clone(), *items, &mut to.clone())
            }
            Operation::Transform { address, from, to } => {
                self.apply_transform(&mut address.clone(), from, to)
            }
        }
    }

    /// Apply an `Add` patch operation
    fn apply_add(&mut self, _address: &mut Address, _value: &Value) -> Result<()> {
        bail!(invalid_patch_operation::<Self>("add"))
    }

    /// Apply a `Remove` patch operation
    fn apply_remove(&mut self, _address: &mut Address, _items: usize) -> Result<()> {
        bail!(invalid_patch_operation::<Self>("remove"))
    }

    /// Apply a `Replace` patch operation
    fn apply_replace(
        &mut self,
        _address: &mut Address,
        _items: usize,
        _value: &Value,
    ) -> Result<()> {
        bail!(invalid_patch_operation::<Self>("replace"))
    }

    /// Apply a `Move` patch operation
    fn apply_move(&mut self, _from: &mut Address, _items: usize, _to: &mut Address) -> Result<()> {
        bail!(invalid_patch_operation::<Self>("move"))
    }

    /// Apply a `Transform` patch operation
    fn apply_transform(&mut self, _address: &mut Address, _from: &str, _to: &str) -> Result<()> {
        bail!(invalid_patch_operation::<Self>("transform"))
    }

    /// Cast a [`Value`] to an instance of the type
    fn from_value(value: &Value) -> Result<Self>
    where
        Self: Clone + DeserializeOwned + Sized + 'static,
    {
        let instance = if let Some(value) = value.downcast_ref::<Self>() {
            value.clone()
        } else if let Some(value) = value.downcast_ref::<serde_json::Value>() {
            if let Ok(value) = serde_json::from_value::<Self>(value.clone()) {
                value
            } else {
                bail!(invalid_patch_value::<Self>())
            }
        } else {
            bail!(invalid_patch_value::<Self>())
        };
        Ok(instance)
    }
}

/// Generate the `is_same` method for a type
macro_rules! patchable_is_same {
    () => {
        fn is_same<Other: Any + Clone>(&self, other: &Other) -> Result<()> {
            if let Some(other) = (other as &dyn Any).downcast_ref::<Self>() {
                self.is_equal(&other)
            } else {
                bail!(Error::NotSame)
            }
        }
    };
}

/// Generate the `diff` method for a type
macro_rules! patchable_diff {
    () => {
        fn diff<Other: Any + Clone + Send>(&self, differ: &mut Differ, other: &Other) {
            if let Some(other) = (other as &dyn Any).downcast_ref::<Self>() {
                self.diff_same(differ, other)
            } else {
                self.diff_other(differ, other)
            }
        }
    };
}

mod prelude;

mod atomics;
#[macro_use]
mod enums;
mod boxes;
mod options;
mod strings;
#[macro_use]
mod structs;
mod vecs;

mod blocks;
mod inlines;
mod nodes;
mod works;

#[allow(dead_code)]
#[derive(JsonSchema)]
enum PatchesSchema {
    Slot(Slot),
    Address(Address),
    Patch(Patch),
    Operation(Operation),
    DomPatch(DomPatch),
    DomOperation(DomOperation),
}

/// Get JSON Schemas for this module
pub fn schemas() -> Result<serde_json::Value> {
    schemas::generate::<PatchesSchema>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{assert_json, assert_json_eq};
    use serde_json::json;
    use stencila_schema::{Article, Emphasis, InlineContent, Integer, Paragraph};

    #[test]
    fn test_same_equal() {
        let int_a: Integer = 1;
        let int_b: Integer = 2;
        let opt_a: Option<Integer> = None;
        let opt_b: Option<Integer> = Some(1);
        let vec_a: Vec<Integer> = vec![1, 2, 3];
        let vec_b: Vec<Integer> = vec![3, 2, 1];

        assert!(same(&int_a, &int_a));
        assert!(!same(&int_a, &int_b));
        assert!(!same(&int_a, &vec_a));
        assert!(!same(&int_a, &opt_a));
        assert!(!same(&vec_a, &vec_b));

        assert!(equal(&int_a, &int_a));
        assert!(!equal(&int_a, &int_b));
        assert!(equal(&opt_a, &opt_a));
        assert!(!equal(&opt_a, &opt_b));
        assert!(equal(&vec_a, &vec_a));
        assert!(!equal(&vec_a, &vec_b));
    }

    #[test]
    fn test_diff_apply() -> Result<()> {
        let empty = Paragraph::default();
        let a = Paragraph {
            content: vec![
                InlineContent::String("word1".to_string()),
                InlineContent::String("word2".to_string()),
            ],
            ..Default::default()
        };
        let b = Paragraph {
            content: vec![
                InlineContent::Emphasis(Emphasis {
                    content: vec![InlineContent::String("word1".to_string())],
                    ..Default::default()
                }),
                InlineContent::String("wotwo".to_string()),
            ],
            ..Default::default()
        };

        // Patching `empty` to `a` should return no difference

        let patch = diff(&empty, &empty);
        assert_json!(patch.ops, []);

        let mut patched = empty.clone();
        apply(&mut patched, &patch)?;
        assert_json_eq!(patched, empty);

        // Patching `empty` to `a` should:
        // - replace all content with the content of `a`

        let patch = diff(&empty, &a);
        assert_json!(
            patch.ops,
            [{
                "type": "Add",
                "address": ["content", 0],
                "value": ["word1", "word2"],
                "length": 2
            }]
        );

        let mut patched = empty.clone();
        apply(&mut patched, &patch)?;
        assert_json_eq!(patched, a);

        // Patching `a` to `b` should:
        // - transform `content[0]` from a string to an `Emphasis`
        // - replace part of `content[1]`

        let patch = diff(&a, &b);
        assert_json!(
            patch.ops,
            [{
                "type": "Transform",
                "address": ["content", 0],
                "from": "String",
                "to": "Emphasis"
            },{
                "type": "Replace",
                "address": ["content", 1, 2],
                "items": 3,
                "value": "two",
                "length": 3
            }]
        );

        let mut patched = a.clone();
        apply(&mut patched, &patch)?;
        assert_json_eq!(patched, b);

        Ok(())
    }

    #[test]
    fn test_dom_patch() {
        // Empty article
        let one = Article {
            content: Some(vec![]),
            ..Default::default()
        };

        // Add an empty paragraph
        let two = Article {
            content: Some(vec![BlockContent::Paragraph(Paragraph::default())]),
            ..Default::default()
        };

        // Add words to the paragraph
        let three = Article {
            content: Some(vec![BlockContent::Paragraph(Paragraph {
                content: vec![
                    InlineContent::String("first".to_string()),
                    InlineContent::String(" second".to_string()),
                ],
                ..Default::default()
            })]),
            ..Default::default()
        };

        // Modify a word
        let four = Article {
            content: Some(vec![BlockContent::Paragraph(Paragraph {
                content: vec![
                    InlineContent::String("foot".to_string()),
                    InlineContent::String(" second".to_string()),
                ],
                ..Default::default()
            })]),
            ..Default::default()
        };

        // Move words
        let five = Article {
            content: Some(vec![BlockContent::Paragraph(Paragraph {
                content: vec![
                    InlineContent::String(" second".to_string()),
                    InlineContent::String("foot".to_string()),
                ],
                ..Default::default()
            })]),
            ..Default::default()
        };

        // one to one -> empty patch
        let patch = diff(&one, &one);
        assert!(patch.ops.is_empty());
        let dom_patch = DomPatch::new(&patch);
        assert!(dom_patch.ops.is_empty());

        // one to two -> `Add` operation on the article's content
        let patch = diff(&one, &two);
        assert_json_eq!(
            patch.ops,
            json!([{
                "type": "Add",
                "address": ["content", 0],
                "value": [{"type": "Paragraph", "content": []}],
                "length": 1
            }])
        );
        let dom_patch = DomPatch::new(&patch);
        assert_json_eq!(
            dom_patch.ops,
            json!([{
                "type": "Add",
                "address": ["content"],
                "html": "<p itemtype=\"http://schema.stenci.la/Paragraph\" itemscope></p>",
                "json": [{"type": "Paragraph", "content": []}]
            }])
        );

        // two to three -> `Add` operation on the paragraph's content
        let patch = diff(&two, &three);
        assert_json_eq!(
            patch.ops,
            json!([{
                "type": "Add",
                "address": ["content", 0, "content", 0],
                "value": ["first", " second"],
                "length": 2
            }])
        );
        let dom_patch = DomPatch::new(&patch);
        assert_json_eq!(
            dom_patch.ops,
            json!([{
                "type": "Add",
                "address": ["content", 0, "content", 0],
                "html": "first second",
                "json": ["first", " second"]
            }])
        );

        // three to four -> `Replace` operation on a word
        let patch = diff(&three, &four);
        assert_json_eq!(
            patch.ops,
            json!([{
                "type": "Replace",
                "address": ["content", 0, "content", 0, 1],
                "items": 3,
                "value": "oo",
                "length": 2
            }])
        );
        let dom_patch = DomPatch::new(&patch);
        assert_json_eq!(
            dom_patch.ops,
            json!([{
                "type": "Replace",
                "address": ["content", 0, "content", 0, 1],
                "items": 3,
                "html": "oo",
                "json": "oo"
            }])
        );

        // four to five -> `Move` operation on the word
        let patch = diff(&four, &five);
        assert_json_eq!(
            patch.ops,
            json!([{
                "type": "Move",
                "from": ["content", 0, "content", 1],
                "items": 1,
                "to": ["content", 0, "content", 0],
            }])
        );
        let dom_patch = DomPatch::new(&patch);
        assert_json_eq!(
            dom_patch.ops,
            json!([{
                "type": "Move",
                "from": ["content", 0, "content", 1],
                "items": 1,
                "to": ["content", 0, "content", 0],
            }])
        );
    }
}
