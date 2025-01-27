# Date Time Validator

**A validator specifying the constraints on a date-time.**

**`@id`**: `stencila:DateTimeValidator`

## Properties

The `DateTimeValidator` type has these properties:

| Name    | Aliases | `@id`                                | Type                                                                                                 | Description                                | Inherited from                                                                                   |
| ------- | ------- | ------------------------------------ | ---------------------------------------------------------------------------------------------------- | ------------------------------------------ | ------------------------------------------------------------------------------------------------ |
| id      | -       | [`schema:id`](https://schema.org/id) | [`String`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/data/string.md)      | The identifier for this item.              | [`Entity`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/other/entity.md) |
| minimum | -       | `stencila:minimum`                   | [`DateTime`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/data/date-time.md) | The inclusive lower limit for a date-time. | -                                                                                                |
| maximum | -       | `stencila:maximum`                   | [`DateTime`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/data/date-time.md) | The inclusive upper limit for a date-time. | -                                                                                                |

## Related

The `DateTimeValidator` type is related to these types:

- Parents: [`Entity`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/other/entity.md)
- Children: none

## Formats

The `DateTimeValidator` type can be encoded (serialized) to, and/or decoded (deserialized) from, these formats:

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

The `DateTimeValidator` type is represented in these bindings:

- [JSON-LD](https://stencila.dev/DateTimeValidator.jsonld)
- [JSON Schema](https://stencila.dev/DateTimeValidator.schema.json)
- Python class [`DateTimeValidator`](https://github.com/stencila/stencila/blob/main/python/python/stencila/types/date_time_validator.py)
- Rust struct [`DateTimeValidator`](https://github.com/stencila/stencila/blob/main/rust/schema/src/types/date_time_validator.rs)
- TypeScript class [`DateTimeValidator`](https://github.com/stencila/stencila/blob/main/typescript/src/types/DateTimeValidator.ts)

## Source

This documentation was generated from [`DateTimeValidator.yaml`](https://github.com/stencila/stencila/blob/main/schema/DateTimeValidator.yaml) by [`docs.rs`](https://github.com/stencila/stencila/blob/main/rust/schema-gen/src/docs.rs).