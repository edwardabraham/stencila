use crate::Object;

use super::prelude::*;

impl HtmlCodec for Object {
    fn to_html(&self) -> String {
        elem(
            &name("Object"),
            &[],
            &[elem(
                "ul",
                &[],
                &self
                    .iter()
                    .map(|(key, value)| elem("li", &[attr("key", key)], &[value.to_html()]))
                    .collect_vec(),
            )],
        )
    }
}