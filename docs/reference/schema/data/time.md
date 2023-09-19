# Time

**A point in time recurring on multiple days**

**`@id`**: [`schema:Time`](https://schema.org/Time)

## Properties

The `Time` type has these properties:

| Name  | `@id`                                      | Type                                                                                            | Description                                                     | Inherited from                                                                                   |
| ----- | ------------------------------------------ | ----------------------------------------------------------------------------------------------- | --------------------------------------------------------------- | ------------------------------------------------------------------------------------------------ |
| id    | [`schema:id`](https://schema.org/id)       | [`String`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/data/string.md) | The identifier for this item                                    | [`Entity`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/other/entity.md) |
| value | [`schema:value`](https://schema.org/value) | [`String`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/data/string.md) | The time of day as a string in format `hh:mm:ss[Z\|(+\|-)hh:mm]`. | [`Time`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/data/time.md)      |

## Related

The `Time` type is related to these types:

- Parents: [`Entity`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/other/entity.md)
- Children: none

## Formats

The `Time` type can be encoded (serialized) to, and/or decoded (deserialized) from, these formats:

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

The `Time` type is represented in these bindings:

- [JSON-LD](https://stencila.dev/Time.jsonld)
- [JSON Schema](https://stencila.dev/Time.schema.json)
- Python class [`Time`](https://github.com/stencila/stencila/blob/main/python/stencila/types/time.py)
- Rust struct [`Time`](https://github.com/stencila/stencila/blob/main/rust/schema/src/types/time.rs)
- TypeScript class [`Time`](https://github.com/stencila/stencila/blob/main/typescript/src/types/Time.ts)

## Source

This documentation was generated from [`Time.yaml`](https://github.com/stencila/stencila/blob/main/schema/Time.yaml) by [`docs.rs`](https://github.com/stencila/stencila/blob/main/rust/schema-gen/src/docs.rs).