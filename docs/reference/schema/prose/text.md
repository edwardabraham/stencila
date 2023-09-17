---
title:
- type: Text
  value: Text
---

# Text

**Textual content**

Intended mostly for use for inline text e.g. the text in a paragraph.

Differs from the primitive `String` type in that it has a `type` and `id` property.
The `id` property allows use to identify text nodes with a sequence of inline nodes
for better diffing.

Also, in Rust, the `value` property is implemented as a CRDT.


**`@id`**: [`schema:Text`](https://schema.org/Text)

## Properties

The `Text` type has these properties:

| Name  | `@id`                                      | Type                                                               | Description                   | Inherited from                                                      |
| ----- | ------------------------------------------ | ------------------------------------------------------------------ | ----------------------------- | ------------------------------------------------------------------- |
| id    | [`schema:id`](https://schema.org/id)       | [`String`](https://stencila.dev/docs/reference/schema/data/string) | The identifier for this item  | [`Entity`](https://stencila.dev/docs/reference/schema/other/entity) |
| value | [`schema:value`](https://schema.org/value) | [`Cord`](https://stencila.dev/docs/reference/schema/data/cord)     | The value of the text content | [`Text`](https://stencila.dev/docs/reference/schema/prose/text)     |

## Related

The `Text` type is related to these types:

- Parents: [`Entity`](https://stencila.dev/docs/reference/schema/other/entity)
- Children: none

## Formats

The `Text` type can be encoded (serialized) to, and/or decoded (deserialized) from, these formats:

| Format                                                           | Encoding      | Decoding     | Status                 | Notes                                                                                     |
| ---------------------------------------------------------------- | ------------- | ------------ | ---------------------- | ----------------------------------------------------------------------------------------- |
| [HTML](https://stencila.dev/docs/reference/formats/{name})       | 🔷 Low loss    |              | 🚧 Under development    | Encoded to tag [`<span>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/span) |
| [JATS](https://stencila.dev/docs/reference/formats/{name})       | 🔷 Low loss    |              | 🚧 Under development    | Encoded using special function                                                            |
| [Markdown](https://stencila.dev/docs/reference/formats/{name})   | 🟢 No loss     |              | 🚧 Under development    |                                                                                           |
| [Plain text](https://stencila.dev/docs/reference/formats/{name}) | 🟢 No loss     |              | 🟥 Alpha                |                                                                                           |
| [JSON](https://stencila.dev/docs/reference/formats/{name})       | 🟢 No loss     | 🟢 No loss    | 🟢 Stable               |                                                                                           |
| [JSON5](https://stencila.dev/docs/reference/formats/{name})      | 🟢 No loss     | 🟢 No loss    | 🟢 Stable               |                                                                                           |
| [YAML](https://stencila.dev/docs/reference/formats/{name})       | 🟢 No loss     | 🟢 No loss    | 🟢 Stable               |                                                                                           |
| [Debug](https://stencila.dev/docs/reference/formats/{name})      | 🔷 Low loss    |              | 🟢 Stable               |                                                                                           |

## Bindings

The `Text` type is represented in these bindings:

- [JSON-LD](https://stencila.dev/Text.jsonld)
- [JSON Schema](https://stencila.dev/Text.schema.json)
- Python class [`Text`](https://github.com/stencila/stencila/blob/main/python/stencila/types/text.py)
- Rust struct [`Text`](https://github.com/stencila/stencila/blob/main/rust/schema/src/types/text.rs)
- TypeScript class [`Text`](https://github.com/stencila/stencila/blob/main/typescript/src/types/Text.ts)

## Source

This documentation was generated from [`Text.yaml`](https://github.com/stencila/stencila/blob/main/schema/Text.yaml) by [`docs.rs`](https://github.com/stencila/stencila/blob/main/rust/schema-gen/src/docs.rs).