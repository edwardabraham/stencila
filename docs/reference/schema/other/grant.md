# Grant

**A grant, typically financial or otherwise quantifiable, of resources.**

**`@id`**: [`schema:Grant`](https://schema.org/Grant)

## Properties

The `Grant` type has these properties:

| Name           | Aliases                                                                         | `@id`                                                      | Type                                                                                                                                                                                                                 | Description                                                                                          | Inherited from                                                                                   |
| -------------- | ------------------------------------------------------------------------------- | ---------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------ |
| id             | -                                                                               | [`schema:id`](https://schema.org/id)                       | [`String`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/data/string.md)                                                                                                                      | The identifier for this item.                                                                        | [`Entity`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/other/entity.md) |
| alternateNames | alternate-names, alternate_names, alternateName, alternate-name, alternate_name | [`schema:alternateName`](https://schema.org/alternateName) | [`String`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/data/string.md)*                                                                                                                     | Alternate names (aliases) for the item.                                                              | [`Thing`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/other/thing.md)   |
| description    | -                                                                               | [`schema:description`](https://schema.org/description)     | [`Text`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/prose/text.md)                                                                                                                         | A description of the item.                                                                           | [`Thing`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/other/thing.md)   |
| identifiers    | identifier                                                                      | [`schema:identifier`](https://schema.org/identifier)       | ([`PropertyValue`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/other/property-value.md) \| [`String`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/data/string.md))* | Any kind of identifier for any kind of Thing.                                                        | [`Thing`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/other/thing.md)   |
| images         | image                                                                           | [`schema:image`](https://schema.org/image)                 | [`ImageObject`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/works/image-object.md)*                                                                                                         | Images of the item.                                                                                  | [`Thing`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/other/thing.md)   |
| name           | -                                                                               | [`schema:name`](https://schema.org/name)                   | [`String`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/data/string.md)                                                                                                                      | The name of the item.                                                                                | [`Thing`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/other/thing.md)   |
| url            | -                                                                               | [`schema:url`](https://schema.org/url)                     | [`String`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/data/string.md)                                                                                                                      | The URL of the item.                                                                                 | [`Thing`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/other/thing.md)   |
| fundedItems    | funded-items, funded_items, fundedItem, funded-item, funded_item                | [`schema:fundedItem`](https://schema.org/fundedItem)       | [`Thing`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/other/thing.md)*                                                                                                                      | Indicates an item funded or sponsored through a Grant.                                               | -                                                                                                |
| sponsors       | sponsor                                                                         | [`schema:sponsor`](https://schema.org/sponsor)             | ([`Person`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/other/person.md) \| [`Organization`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/other/organization.md))*  | A person or organization that supports a thing through a pledge, promise, or financial contribution. | -                                                                                                |

## Related

The `Grant` type is related to these types:

- Parents: [`Thing`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/other/thing.md)
- Children: [`MonetaryGrant`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/other/monetary-grant.md)

## Formats

The `Grant` type can be encoded (serialized) to, and/or decoded (deserialized) from, these formats:

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

The `Grant` type is represented in these bindings:

- [JSON-LD](https://stencila.dev/Grant.jsonld)
- [JSON Schema](https://stencila.dev/Grant.schema.json)
- Python class [`Grant`](https://github.com/stencila/stencila/blob/main/python/python/stencila/types/grant.py)
- Rust struct [`Grant`](https://github.com/stencila/stencila/blob/main/rust/schema/src/types/grant.rs)
- TypeScript class [`Grant`](https://github.com/stencila/stencila/blob/main/typescript/src/types/Grant.ts)

## Source

This documentation was generated from [`Grant.yaml`](https://github.com/stencila/stencila/blob/main/schema/Grant.yaml) by [`docs.rs`](https://github.com/stencila/stencila/blob/main/rust/schema-gen/src/docs.rs).