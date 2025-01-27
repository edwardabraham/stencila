# Thematic Break

**A thematic break, such as a scene change in a story, a transition to another topic, or a new document.**

**`@id`**: `stencila:ThematicBreak`

## Properties

The `ThematicBreak` type has these properties:

| Name | Aliases | `@id`                                | Type                                                                                            | Description                   | Inherited from                                                                                   |
| ---- | ------- | ------------------------------------ | ----------------------------------------------------------------------------------------------- | ----------------------------- | ------------------------------------------------------------------------------------------------ |
| id   | -       | [`schema:id`](https://schema.org/id) | [`String`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/data/string.md) | The identifier for this item. | [`Entity`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/other/entity.md) |

## Related

The `ThematicBreak` type is related to these types:

- Parents: [`Entity`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/other/entity.md)
- Children: none

## Formats

The `ThematicBreak` type can be encoded (serialized) to, and/or decoded (deserialized) from, these formats:

| Format                                                                                        | Encoding         | Decoding     | Status                 | Notes                                                                                              |
| --------------------------------------------------------------------------------------------- | ---------------- | ------------ | ---------------------- | -------------------------------------------------------------------------------------------------- |
| [HTML](https://github.com/stencila/stencila/blob/main/docs/reference/formats/html.md)         | 🟢 No loss        |              | 🚧 Under development    | Encoded to tag [`<hr>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/hr)              |
| [JATS](https://github.com/stencila/stencila/blob/main/docs/reference/formats/jats.md)         | 🟢 No loss        | 🟢 No loss    | 🚧 Under development    | Encoded to tag [`<hr>`](https://jats.nlm.nih.gov/articleauthoring/tag-library/1.3/element/hr.html) |
| [Markdown](https://github.com/stencila/stencila/blob/main/docs/reference/formats/markdown.md) | 🟢 No loss        |              | 🚧 Under development    | Encoded using template `---\n\n`                                                                   |
| [Plain text](https://github.com/stencila/stencila/blob/main/docs/reference/formats/text.md)   | ⚠️ High loss     |              | ⚠️ Alpha               |                                                                                                    |
| [JSON](https://github.com/stencila/stencila/blob/main/docs/reference/formats/json.md)         | 🟢 No loss        | 🟢 No loss    | 🟢 Stable               |                                                                                                    |
| [JSON5](https://github.com/stencila/stencila/blob/main/docs/reference/formats/json5.md)       | 🟢 No loss        | 🟢 No loss    | 🟢 Stable               |                                                                                                    |
| [YAML](https://github.com/stencila/stencila/blob/main/docs/reference/formats/yaml.md)         | 🟢 No loss        | 🟢 No loss    | 🟢 Stable               |                                                                                                    |
| [Debug](https://github.com/stencila/stencila/blob/main/docs/reference/formats/debug.md)       | 🔷 Low loss       |              | 🟢 Stable               |                                                                                                    |

## Bindings

The `ThematicBreak` type is represented in these bindings:

- [JSON-LD](https://stencila.dev/ThematicBreak.jsonld)
- [JSON Schema](https://stencila.dev/ThematicBreak.schema.json)
- Python class [`ThematicBreak`](https://github.com/stencila/stencila/blob/main/python/python/stencila/types/thematic_break.py)
- Rust struct [`ThematicBreak`](https://github.com/stencila/stencila/blob/main/rust/schema/src/types/thematic_break.rs)
- TypeScript class [`ThematicBreak`](https://github.com/stencila/stencila/blob/main/typescript/src/types/ThematicBreak.ts)

## Source

This documentation was generated from [`ThematicBreak.yaml`](https://github.com/stencila/stencila/blob/main/schema/ThematicBreak.yaml) by [`docs.rs`](https://github.com/stencila/stencila/blob/main/rust/schema-gen/src/docs.rs).