# Null

**The null value**

**`@id`**: `stencila:Null`

## Formats

The `Null` type can be encoded (serialized) to, and/or decoded (deserialized) from, these formats:

| Format                                                                                            | Encoding      | Decoding     | Status                 | Notes |
| ------------------------------------------------------------------------------------------------- | ------------- | ------------ | ---------------------- | ----- |
| [HTML](https://github.com/stencila/stencila/blob/main/docs/reference/formats/HTML.md)             | 🔷 Low loss    |              | 🚧 Under development    |       |
| [JATS](https://github.com/stencila/stencila/blob/main/docs/reference/formats/JATS.md)             | 🔷 Low loss    |              | 🚧 Under development    |       |
| [Markdown](https://github.com/stencila/stencila/blob/main/docs/reference/formats/Markdown.md)     | 🔷 Low loss    |              | 🚧 Under development    |       |
| [Plain text](https://github.com/stencila/stencila/blob/main/docs/reference/formats/Plain text.md) | 🔷 Low loss    |              | 🟥 Alpha                |       |
| [JSON](https://github.com/stencila/stencila/blob/main/docs/reference/formats/JSON.md)             | 🟢 No loss     | 🟢 No loss    | 🟢 Stable               |       |
| [JSON5](https://github.com/stencila/stencila/blob/main/docs/reference/formats/JSON5.md)           | 🟢 No loss     | 🟢 No loss    | 🟢 Stable               |       |
| [YAML](https://github.com/stencila/stencila/blob/main/docs/reference/formats/YAML.md)             | 🟢 No loss     | 🟢 No loss    | 🟢 Stable               |       |
| [Debug](https://github.com/stencila/stencila/blob/main/docs/reference/formats/Debug.md)           | 🔷 Low loss    |              | 🟢 Stable               |       |

## Bindings

The `Null` type is represented in these bindings:

- [JSON-LD](https://stencila.dev/Null.jsonld)
- [JSON Schema](https://stencila.dev/Null.schema.json)
- Python type [`Null`](https://github.com/stencila/stencila/blob/main/python/stencila/types/null.py)
- Rust type [`Null`](https://github.com/stencila/stencila/blob/main/rust/schema/src/types/null.rs)
- TypeScript type [`Null`](https://github.com/stencila/stencila/blob/main/typescript/src/types/Null.ts)

## Source

This documentation was generated from [`Null.yaml`](https://github.com/stencila/stencila/blob/main/schema/Null.yaml) by [`docs.rs`](https://github.com/stencila/stencila/blob/main/rust/schema-gen/src/docs.rs).