# Object

**A value comprised of keyed primitive nodes.**

Note that keys are strings and values are restricted to primitive node
types including `Object` (ie. an `Object` as a value of another `Object`) and `Array`.


**`@id`**: `stencila:Object`

## Formats

The `Object` type can be encoded (serialized) to, and/or decoded (deserialized) from, these formats:

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

The `Object` type is represented in these bindings:

- [JSON-LD](https://stencila.dev/Object.jsonld)
- [JSON Schema](https://stencila.dev/Object.schema.json)
- Python type [`Object`](https://github.com/stencila/stencila/blob/main/python/python/stencila/types/object.py)
- Rust type [`Object`](https://github.com/stencila/stencila/blob/main/rust/schema/src/types/object.rs)
- TypeScript type [`Object`](https://github.com/stencila/stencila/blob/main/typescript/src/types/Object.ts)

## Source

This documentation was generated from [`Object.yaml`](https://github.com/stencila/stencila/blob/main/schema/Object.yaml) by [`docs.rs`](https://github.com/stencila/stencila/blob/main/rust/schema-gen/src/docs.rs).