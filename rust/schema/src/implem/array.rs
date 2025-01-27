use codec_html_trait::encode::{attr, elem};
use node_store::{
    automerge::{ObjId, Prop},
    ReadNode, ReadStore, WriteNode, WriteStore,
};

use crate::{prelude::*, Array, Primitive};

impl std::fmt::Display for Array {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Array")
    }
}

impl StripNode for Array {}

impl ReadNode for Array {
    fn load_list<S: ReadStore>(store: &S, obj_id: &ObjId) -> Result<Self> {
        Ok(Self(Vec::<Primitive>::load_list(store, obj_id)?))
    }

    fn load_none() -> Result<Self> {
        Ok(Self(Vec::<Primitive>::load_none()?))
    }
}

impl WriteNode for Array {
    fn insert_prop(&self, store: &mut WriteStore, obj_id: &ObjId, prop: Prop) -> Result<()> {
        self.0.insert_prop(store, obj_id, prop)
    }

    fn put_prop(&self, store: &mut WriteStore, obj_id: &ObjId, prop: Prop) -> Result<()> {
        self.0.put_prop(store, obj_id, prop)
    }

    fn similarity<S: ReadStore>(&self, store: &S, obj_id: &ObjId, prop: Prop) -> Result<usize> {
        self.0.similarity(store, obj_id, prop)
    }
}

impl HtmlCodec for Array {
    fn to_html_parts(&self) -> (&str, Vec<String>, Vec<String>) {
        // Uses spans, rather than say <ol>/<li> because needs to be
        // include e.g for output of a `CodeExpression`
        (
            "span",
            vec![attr("is", "stencila-array")],
            self.iter()
                .map(|value| {
                    elem(
                        "span",
                        &[attr("is", "stencila-array-item")],
                        &[value.to_html()],
                    )
                })
                .collect_vec(),
        )
    }

    fn to_html_attr(&self) -> String {
        serde_json::to_string(self).unwrap_or_default()
    }
}

impl MarkdownCodec for Array {
    fn to_markdown(&self, _context: &MarkdownEncodeContext) -> (String, Losses) {
        self.to_text()
    }
}

impl TextCodec for Array {
    fn to_text(&self) -> (String, Losses) {
        let mut text = String::new();
        let mut losses = Losses::one("Array#");

        for (index, item) in self.iter().enumerate() {
            if index != 0 {
                text.push(' ');
            }

            let (item_text, item_losses) = item.to_text();
            text.push_str(&item_text);
            losses.merge(item_losses);
        }

        if !text.is_empty() {
            text.push(' ');
        }

        (text, losses)
    }
}
