# Null

**The null value.**

**`@id`**: `stencila:Null`

## Formats

The `Null` type can be encoded (serialized) to, and/or decoded (deserialized) from, these formats:

| Format                                                                                        | Encoding      | Decoding     | Status                 | Notes |
| --------------------------------------------------------------------------------------------- | ------------- | ------------ | ---------------------- | ----- |
| [HTML](https://github.com/stencila/stencila/blob/main/docs/reference/formats/html.md)         | 🔷 Low loss    |              | 🚧 Under development    |       |
| [JATS](https://github.com/stencila/stencila/blob/main/docs/reference/formats/jats.md)         | 🔷 Low loss    |              | 🚧 Under development    |       |
| [Markdown](https://github.com/stencila/stencila/blob/main/docs/reference/formats/markdown.md) | 🔷 Low loss    |              | 🚧 Under development    |       |
| [Plain text](https://github.com/stencila/stencila/blob/main/docs/reference/formats/text.md)   | 🔷 Low loss    |              | ⚠️ Alpha               |       |
| [JSON](https://github.com/stencila/stencila/blob/main/docs/reference/formats/json.md)         | 🟢 No loss     | 🟢 No loss    | 🟢 Stable               |       |
| [JSON5](https://github.com/stencila/stencila/blob/main/docs/reference/formats/json5.md)       | 🟢 No loss     | 🟢 No loss    | 🟢 Stable               |       |
| [YAML](https://github.com/stencila/stencila/blob/main/docs/reference/formats/yaml.md)         | 🟢 No loss     | 🟢 No loss    | 🟢 Stable               |       |
| [Debug](https://github.com/stencila/stencila/blob/main/docs/reference/formats/debug.md)       | 🔷 Low loss    |              | 🟢 Stable               |       |

## Bindings

The `Null` type is represented in these bindings:

- [JSON-LD](https://stencila.dev/Null.jsonld)
- [JSON Schema](https://stencila.dev/Null.schema.json)
- Python type [`Null`](https://github.com/stencila/stencila/blob/main/python/python/stencila/types/null.py)
- Rust type [`Null`](https://github.com/stencila/stencila/blob/main/rust/schema/src/types/null.rs)
- TypeScript type [`Null`](https://github.com/stencila/stencila/blob/main/typescript/src/types/Null.ts)

## Source

This documentation was generated from [`Null.yaml`](https://github.com/stencila/stencila/blob/main/schema/Null.yaml) by [`docs.rs`](https://github.com/stencila/stencila/blob/main/rust/schema-gen/src/docs.rs).