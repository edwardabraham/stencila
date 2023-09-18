# String Validator

**A schema specifying constraints on a string node.**

A node will be valid against the schema if it is a string that
meets the schemas `minLength`, `maxLength` and `pattern` properties.
Analogous to the JSON Schema `string` validation [type](https://json-schema.org/draft/2019-09/json-schema-validation.html#rfc.section.6.1.1).


**`@id`**: `stencila:StringValidator`

## Properties

The `StringValidator` type has these properties:

| Name      | `@id`                                | Type                                                                                              | Description                                         | Inherited from                                                                                                     |
| --------- | ------------------------------------ | ------------------------------------------------------------------------------------------------- | --------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------ |
| id        | [`schema:id`](https://schema.org/id) | [`String`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/data/string.md)   | The identifier for this item                        | [`Entity`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/other/entity.md)                   |
| minLength | `stencila:minLength`                 | [`Integer`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/data/integer.md) | The minimum length for a string node.               | [`StringValidator`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/data/string-validator.md) |
| maxLength | `stencila:maxLength`                 | [`Integer`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/data/integer.md) | The maximum length for a string node.               | [`StringValidator`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/data/string-validator.md) |
| pattern   | `stencila:pattern`                   | [`String`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/data/string.md)   | A regular expression that a string node must match. | [`StringValidator`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/data/string-validator.md) |

## Related

The `StringValidator` type is related to these types:

- Parents: [`Entity`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/other/entity.md)
- Children: none

## Formats

The `StringValidator` type can be encoded (serialized) to, and/or decoded (deserialized) from, these formats:

| Format                                                                                            | Encoding       | Decoding     | Status                 | Notes |
| ------------------------------------------------------------------------------------------------- | -------------- | ------------ | ---------------------- | ----- |
| [HTML](https://github.com/stencila/stencila/blob/main/docs/reference/formats/HTML.md)             | 🔷 Low loss     |              | 🚧 Under development    |       |
| [JATS](https://github.com/stencila/stencila/blob/main/docs/reference/formats/JATS.md)             | 🔷 Low loss     |              | 🚧 Under development    |       |
| [Markdown](https://github.com/stencila/stencila/blob/main/docs/reference/formats/Markdown.md)     | 🟥 High loss    |              | 🚧 Under development    |       |
| [Plain text](https://github.com/stencila/stencila/blob/main/docs/reference/formats/Plain text.md) | 🟥 High loss    |              | 🟥 Alpha                |       |
| [JSON](https://github.com/stencila/stencila/blob/main/docs/reference/formats/JSON.md)             | 🟢 No loss      | 🟢 No loss    | 🟢 Stable               |       |
| [JSON5](https://github.com/stencila/stencila/blob/main/docs/reference/formats/JSON5.md)           | 🟢 No loss      | 🟢 No loss    | 🟢 Stable               |       |
| [YAML](https://github.com/stencila/stencila/blob/main/docs/reference/formats/YAML.md)             | 🟢 No loss      | 🟢 No loss    | 🟢 Stable               |       |
| [Debug](https://github.com/stencila/stencila/blob/main/docs/reference/formats/Debug.md)           | 🔷 Low loss     |              | 🟢 Stable               |       |

## Bindings

The `StringValidator` type is represented in these bindings:

- [JSON-LD](https://stencila.dev/StringValidator.jsonld)
- [JSON Schema](https://stencila.dev/StringValidator.schema.json)
- Python class [`StringValidator`](https://github.com/stencila/stencila/blob/main/python/stencila/types/string_validator.py)
- Rust struct [`StringValidator`](https://github.com/stencila/stencila/blob/main/rust/schema/src/types/string_validator.rs)
- TypeScript class [`StringValidator`](https://github.com/stencila/stencila/blob/main/typescript/src/types/StringValidator.ts)

## Source

This documentation was generated from [`StringValidator.yaml`](https://github.com/stencila/stencila/blob/main/schema/StringValidator.yaml) by [`docs.rs`](https://github.com/stencila/stencila/blob/main/rust/schema-gen/src/docs.rs).