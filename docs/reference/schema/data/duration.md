# Duration

**A value that represents the difference between two timestamps.**

**`@id`**: [`schema:Duration`](https://schema.org/Duration)

## Properties

The `Duration` type has these properties:

| Name     | Aliases              | `@id`                                      | Type                                                                                                 | Description                                | Inherited from                                                                                   |
| -------- | -------------------- | ------------------------------------------ | ---------------------------------------------------------------------------------------------------- | ------------------------------------------ | ------------------------------------------------------------------------------------------------ |
| id       | -                    | [`schema:id`](https://schema.org/id)       | [`String`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/data/string.md)      | The identifier for this item.              | [`Entity`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/other/entity.md) |
| value    | -                    | [`schema:value`](https://schema.org/value) | [`Integer`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/data/integer.md)    | The time difference in `timeUnit`s.        | -                                                                                                |
| timeUnit | time-unit, time_unit | `stencila:timeUnit`                        | [`TimeUnit`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/data/time-unit.md) | The time unit that the `value` represents. | -                                                                                                |

## Related

The `Duration` type is related to these types:

- Parents: [`Entity`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/other/entity.md)
- Children: none

## Formats

The `Duration` type can be encoded (serialized) to, and/or decoded (deserialized) from, these formats:

| Format                                                                                        | Encoding         | Decoding     | Status                 | Notes                                                                                                                                 |
| --------------------------------------------------------------------------------------------- | ---------------- | ------------ | ---------------------- | ------------------------------------------------------------------------------------------------------------------------------------- |
| [HTML](https://github.com/stencila/stencila/blob/main/docs/reference/formats/html.md)         | 🔷 Low loss       |              | 🚧 Under development    |                                                                                                                                       |
| [JATS](https://github.com/stencila/stencila/blob/main/docs/reference/formats/jats.md)         | 🟢 No loss        | 🟢 No loss    | 🚧 Under development    | Encoded to tag [`<duration>`](https://jats.nlm.nih.gov/articleauthoring/tag-library/1.3/element/duration.html) using special function |
| [Markdown](https://github.com/stencila/stencila/blob/main/docs/reference/formats/markdown.md) | ⚠️ High loss     |              | 🚧 Under development    |                                                                                                                                       |
| [Plain text](https://github.com/stencila/stencila/blob/main/docs/reference/formats/text.md)   | ⚠️ High loss     |              | ⚠️ Alpha               |                                                                                                                                       |
| [JSON](https://github.com/stencila/stencila/blob/main/docs/reference/formats/json.md)         | 🟢 No loss        | 🟢 No loss    | 🟢 Stable               |                                                                                                                                       |
| [JSON5](https://github.com/stencila/stencila/blob/main/docs/reference/formats/json5.md)       | 🟢 No loss        | 🟢 No loss    | 🟢 Stable               |                                                                                                                                       |
| [YAML](https://github.com/stencila/stencila/blob/main/docs/reference/formats/yaml.md)         | 🟢 No loss        | 🟢 No loss    | 🟢 Stable               |                                                                                                                                       |
| [Debug](https://github.com/stencila/stencila/blob/main/docs/reference/formats/debug.md)       | 🔷 Low loss       |              | 🟢 Stable               |                                                                                                                                       |

## Bindings

The `Duration` type is represented in these bindings:

- [JSON-LD](https://stencila.dev/Duration.jsonld)
- [JSON Schema](https://stencila.dev/Duration.schema.json)
- Python class [`Duration`](https://github.com/stencila/stencila/blob/main/python/python/stencila/types/duration.py)
- Rust struct [`Duration`](https://github.com/stencila/stencila/blob/main/rust/schema/src/types/duration.rs)
- TypeScript class [`Duration`](https://github.com/stencila/stencila/blob/main/typescript/src/types/Duration.ts)

## Source

This documentation was generated from [`Duration.yaml`](https://github.com/stencila/stencila/blob/main/schema/Duration.yaml) by [`docs.rs`](https://github.com/stencila/stencila/blob/main/rust/schema-gen/src/docs.rs).