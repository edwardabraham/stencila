# Code Block

**A code block.**

**`@id`**: `stencila:CodeBlock`

## Properties

The `CodeBlock` type has these properties:

| Name                | `@id`                                                                  | Type                                                                                            | Description                           | Inherited from                                                                                           |
| ------------------- | ---------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------- | ------------------------------------- | -------------------------------------------------------------------------------------------------------- |
| id                  | [`schema:id`](https://schema.org/id)                                   | [`String`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/data/string.md) | The identifier for this item          | [`Entity`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/other/entity.md)         |
| code                | `stencila:code`                                                        | [`Cord`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/data/cord.md)     | The code.                             | [`CodeStatic`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/code/code-static.md) |
| programmingLanguage | [`schema:programmingLanguage`](https://schema.org/programmingLanguage) | [`String`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/data/string.md) | The programming language of the code. | [`CodeStatic`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/code/code-static.md) |

## Related

The `CodeBlock` type is related to these types:

- Parents: [`CodeStatic`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/code/code-static.md)
- Children: none

## Formats

The `CodeBlock` type can be encoded (serialized) to, and/or decoded (deserialized) from, these formats:

| Format                                                                                        | Encoding       | Decoding     | Status                 | Notes                                                                                             |
| --------------------------------------------------------------------------------------------- | -------------- | ------------ | ---------------------- | ------------------------------------------------------------------------------------------------- |
| [HTML](https://github.com/stencila/stencila/blob/main/docs/reference/formats/html.md)         | 🟢 No loss      |              | 🚧 Under development    | Encoded to tag [`<pre>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/pre)           |
| [JATS](https://github.com/stencila/stencila/blob/main/docs/reference/formats/jats.md)         | 🟢 No loss      |              | 🚧 Under development    | Encoded to tag [`<code>`](https://jats.nlm.nih.gov/articleauthoring/tag-library/1.3/element/code) |
| [Markdown](https://github.com/stencila/stencila/blob/main/docs/reference/formats/markdown.md) | 🟢 No loss      |              | 🚧 Under development    | Encoded using special function                                                                    |
| [Plain text](https://github.com/stencila/stencila/blob/main/docs/reference/formats/text.md)   | 🟥 High loss    |              | 🟥 Alpha                |                                                                                                   |
| [JSON](https://github.com/stencila/stencila/blob/main/docs/reference/formats/json.md)         | 🟢 No loss      | 🟢 No loss    | 🟢 Stable               |                                                                                                   |
| [JSON5](https://github.com/stencila/stencila/blob/main/docs/reference/formats/json5.md)       | 🟢 No loss      | 🟢 No loss    | 🟢 Stable               |                                                                                                   |
| [YAML](https://github.com/stencila/stencila/blob/main/docs/reference/formats/yaml.md)         | 🟢 No loss      | 🟢 No loss    | 🟢 Stable               |                                                                                                   |
| [Debug](https://github.com/stencila/stencila/blob/main/docs/reference/formats/debug.md)       | 🔷 Low loss     |              | 🟢 Stable               |                                                                                                   |

## Bindings

The `CodeBlock` type is represented in these bindings:

- [JSON-LD](https://stencila.dev/CodeBlock.jsonld)
- [JSON Schema](https://stencila.dev/CodeBlock.schema.json)
- Python class [`CodeBlock`](https://github.com/stencila/stencila/blob/main/python/stencila/types/code_block.py)
- Rust struct [`CodeBlock`](https://github.com/stencila/stencila/blob/main/rust/schema/src/types/code_block.rs)
- TypeScript class [`CodeBlock`](https://github.com/stencila/stencila/blob/main/typescript/src/types/CodeBlock.ts)

## Source

This documentation was generated from [`CodeBlock.yaml`](https://github.com/stencila/stencila/blob/main/schema/CodeBlock.yaml) by [`docs.rs`](https://github.com/stencila/stencila/blob/main/rust/schema-gen/src/docs.rs).