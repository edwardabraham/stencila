# Strikeout

**Content that is marked as struck out.**

Analogues of `Strikeout` in other schema include:
  - HTML [`<del>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/del)
  - JATS XML [`<strike>`](https://jats.nlm.nih.gov/archiving/tag-library/1.2/element/strike.html)
  - MDAST [`Delete`](https://github.com/syntax-tree/mdast#delete)
  - Pandoc [`Strikeout`](https://github.com/jgm/pandoc-types/blob/1.17.5.4/Text/Pandoc/Definition.hs#L258)
Supersedes the `Delete` inline content type (the name "Strikeout" is less ambiguous than "Delete").


**`@id`**: `stencila:Strikeout`

## Properties

The `Strikeout` type has these properties:

| Name    | Aliases | `@id`                                | Type                                                                                              | Description                   | Inherited from                                                                                   |
| ------- | ------- | ------------------------------------ | ------------------------------------------------------------------------------------------------- | ----------------------------- | ------------------------------------------------------------------------------------------------ |
| id      | -       | [`schema:id`](https://schema.org/id) | [`String`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/data/string.md)   | The identifier for this item. | [`Entity`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/other/entity.md) |
| content | -       | `stencila:content`                   | [`Inline`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/prose/inline.md)* | The content that is marked.   | [`Mark`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/prose/mark.md)     |

## Related

The `Strikeout` type is related to these types:

- Parents: [`Mark`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/prose/mark.md)
- Children: none

## Formats

The `Strikeout` type can be encoded (serialized) to, and/or decoded (deserialized) from, these formats:

| Format                                                                                        | Encoding         | Decoding     | Status                 | Notes                                                                                                      |
| --------------------------------------------------------------------------------------------- | ---------------- | ------------ | ---------------------- | ---------------------------------------------------------------------------------------------------------- |
| [HTML](https://github.com/stencila/stencila/blob/main/docs/reference/formats/html.md)         | 🔷 Low loss       |              | 🚧 Under development    | Encoded to tag [`<s>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/s)                        |
| [JATS](https://github.com/stencila/stencila/blob/main/docs/reference/formats/jats.md)         | 🟢 No loss        | 🟢 No loss    | 🚧 Under development    | Encoded to tag [`<strike>`](https://jats.nlm.nih.gov/articleauthoring/tag-library/1.3/element/strike.html) |
| [Markdown](https://github.com/stencila/stencila/blob/main/docs/reference/formats/markdown.md) | ⚠️ High loss     |              | 🚧 Under development    | Encoded using template `~~{content}~~`                                                                     |
| [Plain text](https://github.com/stencila/stencila/blob/main/docs/reference/formats/text.md)   | ⚠️ High loss     |              | ⚠️ Alpha               |                                                                                                            |
| [JSON](https://github.com/stencila/stencila/blob/main/docs/reference/formats/json.md)         | 🟢 No loss        | 🟢 No loss    | 🟢 Stable               |                                                                                                            |
| [JSON5](https://github.com/stencila/stencila/blob/main/docs/reference/formats/json5.md)       | 🟢 No loss        | 🟢 No loss    | 🟢 Stable               |                                                                                                            |
| [YAML](https://github.com/stencila/stencila/blob/main/docs/reference/formats/yaml.md)         | 🟢 No loss        | 🟢 No loss    | 🟢 Stable               |                                                                                                            |
| [Debug](https://github.com/stencila/stencila/blob/main/docs/reference/formats/debug.md)       | 🔷 Low loss       |              | 🟢 Stable               |                                                                                                            |

## Bindings

The `Strikeout` type is represented in these bindings:

- [JSON-LD](https://stencila.dev/Strikeout.jsonld)
- [JSON Schema](https://stencila.dev/Strikeout.schema.json)
- Python class [`Strikeout`](https://github.com/stencila/stencila/blob/main/python/python/stencila/types/strikeout.py)
- Rust struct [`Strikeout`](https://github.com/stencila/stencila/blob/main/rust/schema/src/types/strikeout.rs)
- TypeScript class [`Strikeout`](https://github.com/stencila/stencila/blob/main/typescript/src/types/Strikeout.ts)

## Testing

During property-based (a.k.a generative) testing, the properties of the `Strikeout` type are generated using the following strategies for each complexity level (see the [`proptest` book](https://proptest-rs.github.io/proptest/) for an explanation of the Rust strategy expressions). Any optional properties that are not in this table are set to `None`.

| Property  | Complexity | Description                                                | Strategy                                                                |
| --------- | ---------- | ---------------------------------------------------------- | ----------------------------------------------------------------------- |
| `content` | Min+       | Generate a single fixed text value.                        | `vec(Just(Inline::Text(crate::Text::from("text"))), size_range(1..=1))` |
|           | Low+       | Generate a single arbitrary, non-recursive, inline node    | `vec_inlines_non_recursive(1)`                                          |
|           | High+      | Generate up to two arbitrary, non-recursive, inline nodes  | `vec_inlines_non_recursive(2)`                                          |
|           | Max        | Generate up to four arbitrary, non-recursive, inline nodes | `vec_inlines_non_recursive(4)`                                          |

## Source

This documentation was generated from [`Strikeout.yaml`](https://github.com/stencila/stencila/blob/main/schema/Strikeout.yaml) by [`docs.rs`](https://github.com/stencila/stencila/blob/main/rust/schema-gen/src/docs.rs).