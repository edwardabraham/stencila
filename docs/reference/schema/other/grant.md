---
title:
- type: Text
  value: Grant
---

# Grant

**A grant, typically financial or otherwise quantifiable, of resources.**

**`@id`**: [`schema:Grant`](https://schema.org/Grant)

## Properties

The `Grant` type has these properties:

| Name           | `@id`                                                      | Type                                                                                                                                                       | Description                                                                                          | Inherited from                                                      |
| -------------- | ---------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------- |
| id             | [`schema:id`](https://schema.org/id)                       | [`String`](https://stencila.dev/docs/reference/schema/data/string)                                                                                         | The identifier for this item                                                                         | [`Entity`](https://stencila.dev/docs/reference/schema/other/entity) |
| alternateNames | [`schema:alternateName`](https://schema.org/alternateName) | [`String`](https://stencila.dev/docs/reference/schema/data/string)*                                                                                        | Alternate names (aliases) for the item.                                                              | [`Thing`](https://stencila.dev/docs/reference/schema/other/thing)   |
| description    | [`schema:description`](https://schema.org/description)     | [`Block`](https://stencila.dev/docs/reference/schema/prose/block)*                                                                                         | A description of the item.                                                                           | [`Thing`](https://stencila.dev/docs/reference/schema/other/thing)   |
| identifiers    | [`schema:identifier`](https://schema.org/identifier)       | ([`PropertyValue`](https://stencila.dev/docs/reference/schema/other/property-value) \| [`String`](https://stencila.dev/docs/reference/schema/data/string))* | Any kind of identifier for any kind of Thing.                                                        | [`Thing`](https://stencila.dev/docs/reference/schema/other/thing)   |
| images         | [`schema:image`](https://schema.org/image)                 | ([`ImageObject`](https://stencila.dev/docs/reference/schema/works/image-object) \| [`String`](https://stencila.dev/docs/reference/schema/data/string))*    | Images of the item.                                                                                  | [`Thing`](https://stencila.dev/docs/reference/schema/other/thing)   |
| name           | [`schema:name`](https://schema.org/name)                   | [`String`](https://stencila.dev/docs/reference/schema/data/string)                                                                                         | The name of the item.                                                                                | [`Thing`](https://stencila.dev/docs/reference/schema/other/thing)   |
| url            | [`schema:url`](https://schema.org/url)                     | [`String`](https://stencila.dev/docs/reference/schema/data/string)                                                                                         | The URL of the item.                                                                                 | [`Thing`](https://stencila.dev/docs/reference/schema/other/thing)   |
| fundedItems    | [`schema:fundedItem`](https://schema.org/fundedItem)       | [`Thing`](https://stencila.dev/docs/reference/schema/other/thing)*                                                                                         | Indicates an item funded or sponsored through a Grant.                                               | [`Grant`](https://stencila.dev/docs/reference/schema/other/grant)   |
| sponsors       | [`schema:sponsor`](https://schema.org/sponsor)             | ([`Person`](https://stencila.dev/docs/reference/schema/other/person) \| [`Organization`](https://stencila.dev/docs/reference/schema/other/organization))*  | A person or organization that supports a thing through a pledge, promise, or financial contribution. | [`Grant`](https://stencila.dev/docs/reference/schema/other/grant)   |

## Related

The `Grant` type is related to these types:

- Parents: [`Thing`](https://stencila.dev/docs/reference/schema/other/thing)
- Children: [`MonetaryGrant`](https://stencila.dev/docs/reference/schema/other/monetary-grant)

## Formats

The `Grant` type can be encoded (serialized) to, and/or decoded (deserialized) from, these formats:

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

The `Grant` type is represented in these bindings:

- [JSON-LD](https://stencila.dev/Grant.jsonld)
- [JSON Schema](https://stencila.dev/Grant.schema.json)
- Python class [`Grant`](https://github.com/stencila/stencila/blob/main/python/stencila/types/grant.py)
- Rust struct [`Grant`](https://github.com/stencila/stencila/blob/main/rust/schema/src/types/grant.rs)
- TypeScript class [`Grant`](https://github.com/stencila/stencila/blob/main/typescript/src/types/Grant.ts)

## Source

This documentation was generated from [`Grant.yaml`](https://github.com/stencila/stencila/blob/main/schema/Grant.yaml) by [`docs.rs`](https://github.com/stencila/stencila/blob/main/rust/schema-gen/src/docs.rs).