---
title:
- type: Text
  value: DatatableColumn
---

# Datatable Column

**A column of data within a Datatable.**

**`@id`**: `stencila:DatatableColumn`

## Properties

The `DatatableColumn` type has these properties:

| Name           | `@id`                                                      | Type                                                                                                                                                       | Description                                          | Inherited from                                                                        |
| -------------- | ---------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------- | ------------------------------------------------------------------------------------- |
| id             | [`schema:id`](https://schema.org/id)                       | [`String`](https://stencila.dev/docs/reference/schema/data/string)                                                                                         | The identifier for this item                         | [`Entity`](https://stencila.dev/docs/reference/schema/other/entity)                   |
| alternateNames | [`schema:alternateName`](https://schema.org/alternateName) | [`String`](https://stencila.dev/docs/reference/schema/data/string)*                                                                                        | Alternate names (aliases) for the item.              | [`Thing`](https://stencila.dev/docs/reference/schema/other/thing)                     |
| description    | [`schema:description`](https://schema.org/description)     | [`Block`](https://stencila.dev/docs/reference/schema/prose/block)*                                                                                         | A description of the item.                           | [`Thing`](https://stencila.dev/docs/reference/schema/other/thing)                     |
| identifiers    | [`schema:identifier`](https://schema.org/identifier)       | ([`PropertyValue`](https://stencila.dev/docs/reference/schema/other/property-value) \| [`String`](https://stencila.dev/docs/reference/schema/data/string))* | Any kind of identifier for any kind of Thing.        | [`Thing`](https://stencila.dev/docs/reference/schema/other/thing)                     |
| images         | [`schema:image`](https://schema.org/image)                 | ([`ImageObject`](https://stencila.dev/docs/reference/schema/works/image-object) \| [`String`](https://stencila.dev/docs/reference/schema/data/string))*    | Images of the item.                                  | [`Thing`](https://stencila.dev/docs/reference/schema/other/thing)                     |
| name           | [`schema:name`](https://schema.org/name)                   | [`String`](https://stencila.dev/docs/reference/schema/data/string)                                                                                         | The name of the item.                                | [`Thing`](https://stencila.dev/docs/reference/schema/other/thing)                     |
| url            | [`schema:url`](https://schema.org/url)                     | [`String`](https://stencila.dev/docs/reference/schema/data/string)                                                                                         | The URL of the item.                                 | [`Thing`](https://stencila.dev/docs/reference/schema/other/thing)                     |
| values         | `stencila:values`                                          | [`Primitive`](https://stencila.dev/docs/reference/schema/data/primitive)*                                                                                  | The data values of the column.                       | [`DatatableColumn`](https://stencila.dev/docs/reference/schema/data/datatable-column) |
| validator      | `stencila:validator`                                       | [`ArrayValidator`](https://stencila.dev/docs/reference/schema/data/array-validator)                                                                        | The validator to use to validate data in the column. | [`DatatableColumn`](https://stencila.dev/docs/reference/schema/data/datatable-column) |

## Related

The `DatatableColumn` type is related to these types:

- Parents: [`Thing`](https://stencila.dev/docs/reference/schema/other/thing)
- Children: none

## Formats

The `DatatableColumn` type can be encoded (serialized) to, and/or decoded (deserialized) from, these formats:

| Format                                                           | Encoding       | Decoding     | Status                 | Notes |
| ---------------------------------------------------------------- | -------------- | ------------ | ---------------------- | ----- |
| [HTML](https://stencila.dev/docs/reference/formats/{name})       | 🔷 Low loss     |              | 🚧 Under development    |       |
| [Markdown](https://stencila.dev/docs/reference/formats/{name})   | 🟥 High loss    |              | 🚧 Under development    |       |
| [Plain text](https://stencila.dev/docs/reference/formats/{name}) | 🟥 High loss    |              | 🟥 Alpha                |       |
| [JSON](https://stencila.dev/docs/reference/formats/{name})       | 🟢 No loss      | 🟢 No loss    | 🟢 Stable               |       |
| [JSON5](https://stencila.dev/docs/reference/formats/{name})      | 🟢 No loss      | 🟢 No loss    | 🟢 Stable               |       |
| [YAML](https://stencila.dev/docs/reference/formats/{name})       | 🟢 No loss      | 🟢 No loss    | 🟢 Stable               |       |
| [Debug](https://stencila.dev/docs/reference/formats/{name})      | 🔷 Low loss     |              | 🟢 Stable               |       |

## Bindings

The `DatatableColumn` type is represented in these bindings:

- [JSON-LD](https://stencila.dev/DatatableColumn.jsonld)
- [JSON Schema](https://stencila.dev/DatatableColumn.schema.json)
- Python class [`DatatableColumn`](https://github.com/stencila/stencila/blob/main/python/stencila/types/datatable_column.py)
- Rust struct [`DatatableColumn`](https://github.com/stencila/stencila/blob/main/rust/schema/src/types/datatable_column.rs)
- TypeScript class [`DatatableColumn`](https://github.com/stencila/stencila/blob/main/typescript/src/types/DatatableColumn.ts)

## Source

This documentation was generated from [`DatatableColumn.yaml`](https://github.com/stencila/stencila/blob/main/schema/DatatableColumn.yaml) by [`docs.rs`](https://github.com/stencila/stencila/blob/main/rust/schema-gen/src/docs.rs).