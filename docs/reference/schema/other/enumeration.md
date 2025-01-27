# Enumeration

**Lists or enumerations, for example, a list of cuisines or music genres, etc.**

**`@id`**: [`schema:Enumeration`](https://schema.org/Enumeration)

## Properties

The `Enumeration` type has these properties:

| Name           | Aliases                                                                         | `@id`                                                      | Type                                                                                                                                                                                                                 | Description                                   | Inherited from                                                                                   |
| -------------- | ------------------------------------------------------------------------------- | ---------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------- | ------------------------------------------------------------------------------------------------ |
| id             | -                                                                               | [`schema:id`](https://schema.org/id)                       | [`String`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/data/string.md)                                                                                                                      | The identifier for this item.                 | [`Entity`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/other/entity.md) |
| alternateNames | alternate-names, alternate_names, alternateName, alternate-name, alternate_name | [`schema:alternateName`](https://schema.org/alternateName) | [`String`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/data/string.md)*                                                                                                                     | Alternate names (aliases) for the item.       | [`Thing`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/other/thing.md)   |
| description    | -                                                                               | [`schema:description`](https://schema.org/description)     | [`Text`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/prose/text.md)                                                                                                                         | A description of the item.                    | [`Thing`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/other/thing.md)   |
| identifiers    | identifier                                                                      | [`schema:identifier`](https://schema.org/identifier)       | ([`PropertyValue`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/other/property-value.md) \| [`String`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/data/string.md))* | Any kind of identifier for any kind of Thing. | [`Thing`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/other/thing.md)   |
| images         | image                                                                           | [`schema:image`](https://schema.org/image)                 | [`ImageObject`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/works/image-object.md)*                                                                                                         | Images of the item.                           | [`Thing`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/other/thing.md)   |
| name           | -                                                                               | [`schema:name`](https://schema.org/name)                   | [`String`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/data/string.md)                                                                                                                      | The name of the item.                         | [`Thing`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/other/thing.md)   |
| url            | -                                                                               | [`schema:url`](https://schema.org/url)                     | [`String`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/data/string.md)                                                                                                                      | The URL of the item.                          | [`Thing`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/other/thing.md)   |

## Related

The `Enumeration` type is related to these types:

- Parents: [`Thing`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/other/thing.md)
- Children: [`AutomaticExecution`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/flow/automatic-execution.md), [`CitationIntent`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/prose/citation-intent.md), [`CitationMode`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/prose/citation-mode.md), [`ClaimType`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/works/claim-type.md), [`ExecutionDependantRelation`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/flow/execution-dependant-relation.md), [`ExecutionDependencyRelation`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/flow/execution-dependency-relation.md), [`ExecutionRequired`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/flow/execution-required.md), [`ExecutionStatus`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/flow/execution-status.md), [`FormDeriveAction`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/flow/form-derive-action.md), [`ListOrder`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/prose/list-order.md), [`NoteType`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/prose/note-type.md), [`TableCellType`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/works/table-cell-type.md), [`TableRowType`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/works/table-row-type.md), [`TimeUnit`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/data/time-unit.md)

## Formats

The `Enumeration` type can be encoded (serialized) to, and/or decoded (deserialized) from, these formats:

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

The `Enumeration` type is represented in these bindings:

- [JSON-LD](https://stencila.dev/Enumeration.jsonld)
- [JSON Schema](https://stencila.dev/Enumeration.schema.json)
- Python class [`Enumeration`](https://github.com/stencila/stencila/blob/main/python/python/stencila/types/enumeration.py)
- Rust struct [`Enumeration`](https://github.com/stencila/stencila/blob/main/rust/schema/src/types/enumeration.rs)
- TypeScript class [`Enumeration`](https://github.com/stencila/stencila/blob/main/typescript/src/types/Enumeration.ts)

## Source

This documentation was generated from [`Enumeration.yaml`](https://github.com/stencila/stencila/blob/main/schema/Enumeration.yaml) by [`docs.rs`](https://github.com/stencila/stencila/blob/main/rust/schema-gen/src/docs.rs).