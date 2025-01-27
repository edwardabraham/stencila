# Code Error

**An error that occurred when parsing, compiling or executing a `Code` node.**

**`@id`**: `stencila:CodeError`

This type is marked as unstable and is subject to change.

## Properties

The `CodeError` type has these properties:

| Name         | Aliases                               | `@id`                                | Type                                                                                            | Description                                                | Inherited from                                                                                   |
| ------------ | ------------------------------------- | ------------------------------------ | ----------------------------------------------------------------------------------------------- | ---------------------------------------------------------- | ------------------------------------------------------------------------------------------------ |
| id           | -                                     | [`schema:id`](https://schema.org/id) | [`String`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/data/string.md) | The identifier for this item.                              | [`Entity`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/other/entity.md) |
| errorMessage | message, error-message, error_message | `stencila:errorMessage`              | [`String`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/data/string.md) | The error message or brief description of the error.       | -                                                                                                |
| errorType    | error-type, error_type                | `stencila:errorType`                 | [`String`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/data/string.md) | The type of error e.g. "SyntaxError", "ZeroDivisionError". | -                                                                                                |
| stackTrace   | trace, stack-trace, stack_trace       | `stencila:stackTrace`                | [`String`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/data/string.md) | Stack trace leading up to the error.                       | -                                                                                                |

## Related

The `CodeError` type is related to these types:

- Parents: [`Entity`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/other/entity.md)
- Children: none

## Formats

The `CodeError` type can be encoded (serialized) to, and/or decoded (deserialized) from, these formats:

| Format                                                                                        | Encoding         | Decoding     | Status                 | Notes |
| --------------------------------------------------------------------------------------------- | ---------------- | ------------ | ---------------------- | ----- |
| [HTML](https://github.com/stencila/stencila/blob/main/docs/reference/formats/html.md)         | 🔷 Low loss       |              | 🚧 Under development    |       |
| [JATS](https://github.com/stencila/stencila/blob/main/docs/reference/formats/jats.md)         |                  |              | 🚧 Under development    |       |
| [Markdown](https://github.com/stencila/stencila/blob/main/docs/reference/formats/markdown.md) | ⚠️ High loss     |              | 🚧 Under development    |       |
| [Plain text](https://github.com/stencila/stencila/blob/main/docs/reference/formats/text.md)   | ⚠️ High loss     |              | ⚠️ Alpha               |       |
| [JSON](https://github.com/stencila/stencila/blob/main/docs/reference/formats/json.md)         | 🟢 No loss        | 🟢 No loss    | 🟢 Stable               |       |
| [JSON5](https://github.com/stencila/stencila/blob/main/docs/reference/formats/json5.md)       | 🟢 No loss        | 🟢 No loss    | 🟢 Stable               |       |
| [YAML](https://github.com/stencila/stencila/blob/main/docs/reference/formats/yaml.md)         | 🟢 No loss        | 🟢 No loss    | 🟢 Stable               |       |
| [Debug](https://github.com/stencila/stencila/blob/main/docs/reference/formats/debug.md)       | 🔷 Low loss       |              | 🟢 Stable               |       |

## Bindings

The `CodeError` type is represented in these bindings:

- [JSON-LD](https://stencila.dev/CodeError.jsonld)
- [JSON Schema](https://stencila.dev/CodeError.schema.json)
- Python class [`CodeError`](https://github.com/stencila/stencila/blob/main/python/python/stencila/types/code_error.py)
- Rust struct [`CodeError`](https://github.com/stencila/stencila/blob/main/rust/schema/src/types/code_error.rs)
- TypeScript class [`CodeError`](https://github.com/stencila/stencila/blob/main/typescript/src/types/CodeError.ts)

## Source

This documentation was generated from [`CodeError.yaml`](https://github.com/stencila/stencila/blob/main/schema/CodeError.yaml) by [`docs.rs`](https://github.com/stencila/stencila/blob/main/rust/schema-gen/src/docs.rs).