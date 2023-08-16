//! Generation of Typescript types from Stencila Schema

use std::{
    collections::HashSet,
    fs::read_dir,
    path::{Path, PathBuf},
};

use common::{
    async_recursion::async_recursion,
    eyre::{bail, Context, Report, Result},
    futures::future::try_join_all,
    inflector::Inflector,
    itertools::Itertools,
    tokio::fs::{create_dir_all, remove_file, write},
};

use crate::schemas::{Items, Schema, Schemas, Type, Value};

/// Modules that should not be generated
///
/// These modules are manually written, usually because they are
/// an alias for a native JavasScript type.
const NO_GENERATE_MODULE: &[&str] = &[
    "Array",
    "Boolean",
    "Integer",
    "Null",
    "Number",
    "Object",
    "String",
    "TextValue",
    "UnsignedInteger",
];

impl Schemas {
    /// Generate a TypeScript module for each schema
    pub async fn typescript(&self) -> Result<()> {
        eprintln!("Generating TypeScript types");

        // The top level destination
        let dest = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../typescript/src");
        let dest = dest
            .canonicalize()
            .context(format!("can not find directory `{}`", dest.display()))?;

        // The types directory that modules get generated into
        let types = dest.join("types");
        if types.exists() {
            // Already exists, so clean up existing files, except for those that are not generated
            for file in read_dir(&types)?.flatten() {
                let path = file.path();

                if NO_GENERATE_MODULE.contains(
                    &path
                        .file_name()
                        .unwrap()
                        .to_string_lossy()
                        .strip_suffix(".ts")
                        .unwrap(),
                ) {
                    continue;
                }

                remove_file(&path).await?
            }
        } else {
            // Doesn't exist, so create it
            create_dir_all(&types).await?;
        }

        // Create a module for each schema
        let futures = self
            .schemas
            .values()
            .map(|schema| Self::typescript_module(&types, schema));
        try_join_all(futures).await?;

        // Create an index.ts which export types from all modules (including those
        // that are not generated)
        let exports = read_dir(&types)
            .wrap_err(format!("unable to read directory `{}`", types.display()))?
            .flatten()
            .map(|entry| {
                entry
                    .path()
                    .file_name()
                    .unwrap()
                    .to_string_lossy()
                    .strip_suffix(".ts")
                    .unwrap()
                    .to_string()
            })
            .sorted()
            .map(|module| format!("export * from './types/{module}';"))
            .join("\n");
        write(
            dest.join("index.ts"),
            format!("// Generated file. Do not edit; see `rust/schema-gen` crate.\n\n{exports}\n"),
        )
        .await?;

        Ok(())
    }

    /// Generate a TypeScript module for a schema
    async fn typescript_module(dest: &Path, schema: &Schema) -> Result<()> {
        let Some(title) = &schema.title else {
            bail!("Schema has no title");
        };

        if NO_GENERATE_MODULE.contains(&title.as_str()) || schema.r#abstract {
            return Ok(());
        }

        if schema.any_of.is_some() {
            Self::typescript_any_of(dest, schema).await?;
        } else if schema.r#type.is_none() {
            Self::typescript_struct(dest, title, schema).await?;
        }

        Ok(())
    }

    /// Generate a TypeScript type for a schema
    ///
    /// Returns the name of the type and whether it is a vector and whether it is
    /// a type (rather than an enum variant). Used in the functions below.
    #[async_recursion]
    async fn typescript_type(dest: &Path, schema: &Schema) -> Result<(String, bool, bool)> {
        let result = if let Some(r#type) = &schema.r#type {
            match r#type {
                Type::Array => {
                    let items = match &schema.items {
                        Some(Items::Ref(inner)) => inner.r#ref.to_string(),
                        Some(Items::Type(inner)) => inner.r#type.to_class_case(),
                        Some(Items::AnyOf(inner)) => {
                            let schema = Schema {
                                any_of: Some(inner.any_of.clone()),
                                ..Default::default()
                            };
                            Self::typescript_type(dest, &schema).await?.0
                        }
                        Some(Items::List(inner)) => {
                            let schema = Schema {
                                any_of: Some(inner.clone()),
                                ..Default::default()
                            };
                            Self::typescript_type(dest, &schema).await?.0
                        }
                        None => "Unhandled".to_string(),
                    };
                    (items, true, true)
                }
                _ => (r#type.as_ref().to_string(), false, true),
            }
        } else if let Some(r#ref) = &schema.r#ref {
            (r#ref.to_string(), false, true)
        } else if schema.any_of.is_some() {
            (Self::typescript_any_of(dest, schema).await?, false, true)
        } else if let Some(title) = &schema.title {
            (title.to_string(), false, true)
        } else if let Some(r#const) = &schema.r#const {
            (Self::typescript_value(r#const), false, false)
        } else {
            ("Unhandled".to_string(), false, true)
        };
        Ok(result)
    }

    /// Generate a TypeScript class for a 'struct' schema (having 'properties')
    async fn typescript_struct(dest: &Path, title: &String, schema: &Schema) -> Result<String> {
        let path = dest.join(format!("{}.ts", title));
        if path.exists() {
            return Ok(title.to_string());
        }

        let description = schema
            .description
            .as_ref()
            .unwrap_or(title)
            .trim_end_matches('\n')
            .replace('\n', "\n  // ");

        let mut props = Vec::new();
        let mut required_props = Vec::new();
        let mut used_types = HashSet::new();
        for (name, property) in schema.properties.iter().flatten() {
            let description = property
                .description
                .as_ref()
                .unwrap_or(name)
                .trim_end_matches('\n')
                .replace('\n', "\n  // ");

            let name = name.to_camel_case();

            // Early return for "type" property
            if name == "type" {
                props.push(format!("  // {description}\n  type = \"{title}\";"));
                continue;
            }

            let mut prop = name.clone();

            // Determine Typescript type of the property
            let (mut prop_type, is_array, ..) = Self::typescript_type(dest, property).await?;
            used_types.insert(prop_type.clone());

            // Is the property optional?
            if !property.is_required {
                prop.push('?');
            }

            prop.push_str(": ");

            // Is the property an array?
            if is_array {
                prop_type.push_str("[]");
            };

            prop.push_str(&prop_type);

            // If the property is required, add it to the constructor args.
            if property.is_required {
                // An argument can not be named `arguments` so deal with that
                // special case here.
                required_props.push(if name == "arguments" {
                    (
                        format!("this.{name} = args;"),
                        format!("args: {prop_type}, "),
                    )
                } else {
                    (
                        format!("this.{name} = {name};"),
                        format!("{name}: {prop_type}, "),
                    )
                });
            }

            // Does the field have a default?
            if let Some(default) = property.default.as_ref() {
                let default = Self::typescript_value(default);
                prop.push_str(&format!(" = {default}"));
                if default == "Null" {
                    used_types.insert(default);
                }
            };

            props.push(format!("  // {description}\n  {prop};"));
        }
        let props = props.join("\n\n");
        let required_args = required_props.iter().map(|(.., arg)| arg).join("");
        let required_assignments = required_props
            .iter()
            .map(|(assignment, ..)| assignment)
            .join("\n    ");

        let mut imports = used_types
            .iter()
            .filter(|used_type| *used_type != title)
            .sorted()
            .map(|used_type| format!("import {{ {used_type} }} from './{used_type}';"))
            .join("\n");
        if !imports.is_empty() {
            imports.push_str("\n\n");
        }

        write(
            path,
            &format!(
                r#"// Generated file. Do not edit; see `rust/schema-gen` crate.

{imports}// {description}
export class {title} {{
{props}

  constructor({required_args}options?: {title}) {{
    if (options) Object.assign(this, options)
    {required_assignments}
  }}
}}
"#
            ),
        )
        .await?;

        Ok(title.to_string())
    }

    /// Generate a TypeScript enum for an `anyOf` root schema or property schema
    ///
    /// Returns the name of the generated enum.
    async fn typescript_any_of(dest: &Path, schema: &Schema) -> Result<String> {
        let Some(any_of) = &schema.any_of else {
            bail!("Schema has no anyOf");
        };

        let (alternatives, are_types): (Vec<_>, Vec<_>) =
            try_join_all(any_of.iter().map(|schema| async {
                let (typ, is_array, is_type) = Self::typescript_type(dest, schema).await?;
                let typ = if is_array {
                    Self::typescript_array_of(dest, &typ).await?
                } else {
                    typ
                };
                Ok::<_, Report>((typ, is_type))
            }))
            .await?
            .into_iter()
            .unzip();

        let name = schema
            .title
            .clone()
            .unwrap_or_else(|| alternatives.join("Or"));

        let path = dest.join(format!("{}.ts", name));
        if path.exists() {
            return Ok(name);
        }

        let description = if let Some(title) = &schema.title {
            schema
                .description
                .clone()
                .unwrap_or(title.clone())
                .trim_end_matches('\n')
                .replace('\n', "\n  // ")
        } else {
            alternatives
                .iter()
                .map(|variant| format!("`{variant}`"))
                .join(" or ")
        };

        let alternatives = alternatives
            .into_iter()
            .zip(are_types.into_iter())
            .collect_vec();

        let mut imports = alternatives
            .iter()
            .sorted()
            .filter_map(|(name, is_type)| {
                is_type.then_some(format!("import {{ {name} }} from './{name}'",))
            })
            .join("\n");
        if !imports.is_empty() {
            imports.push_str("\n\n");
        }

        let variants = alternatives
            .into_iter()
            .map(|(variant, is_type)| {
                if is_type {
                    variant
                } else {
                    format!("'{variant}'")
                }
            })
            .join(" |\n  ");

        let rust = format!(
            r#"// Generated file. Do not edit; see `rust/schema-gen` crate.\n\n
            
{imports}// {description}
export type {name} =
  {variants};
"#
        );
        write(path, rust).await?;

        Ok(name)
    }

    /// Generate a TypeScript `type` for an "array of" type
    ///
    /// Returns the name of the generated type (will the plural of the
    /// type of the type of the array items).
    async fn typescript_array_of(dest: &Path, typ: &str) -> Result<String> {
        let name = typ.to_plural();

        let path = dest.join(format!("{}.ts", name));
        if path.exists() {
            return Ok(name);
        }

        let rust = format!(
            r#"// Generated file. Do not edit; see `rust/schema-gen` crate.
            
import {{ {typ} }} from './{typ}';

export type {name} = {typ}[];
"#
        );
        write(path, rust).await?;

        Ok(name)
    }

    /// Generate a TypeScript representation of a JSON schema value
    ///
    /// Returns the name of the type.
    fn typescript_value(value: &Value) -> String {
        match value {
            Value::Null => "null".to_string(),
            Value::Boolean(inner) => inner.to_string(),
            Value::Integer(inner) => inner.to_string(),
            Value::Number(inner) => inner.to_string(),
            Value::String(inner) => inner.to_string(),
            _ => "Unhandled value type".to_string(),
        }
    }
}
