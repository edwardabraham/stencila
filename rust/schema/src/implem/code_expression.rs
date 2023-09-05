use codec_json5_trait::Json5Codec;

use crate::{prelude::*, CodeExpression};

impl CodeExpression {
    pub fn to_markdown_special(&self) -> (String, Losses) {
        let mut md = ["`", &self.code, "`{"].concat();

        if !self.programming_language.is_empty() && self.guess_language != Some(true) {
            md.push_str(&self.programming_language);
            md.push(' ');
        }

        md.push_str("exec");

        if let Some(execution_auto) = &self.options.execution_auto {
            md.push_str(" auto=");
            md.push_str(&execution_auto.to_string().to_lowercase())
        }

        if let Some(output) = self
            .output
            .as_ref()
            .and_then(|output| output.to_json5().ok())
        {
            md.push_str(" output=");
            md.push_str(&output);
        }

        md.push('}');

        // TODO: Losses should include derived, execution related properties
        let losses = if self.id.is_some() {
            Losses::of_id("CodeExpression")
        } else {
            Losses::none()
        };

        (md, losses)
    }
}