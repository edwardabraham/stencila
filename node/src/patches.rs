use crate::prelude::*;
use neon::prelude::*;
use stencila::patches;

/// Get the module's schemas
pub fn schemas(cx: FunctionContext) -> JsResult<JsString> {
    let schemas = patches::schemas();
    to_json_or_throw(cx, schemas)
}
