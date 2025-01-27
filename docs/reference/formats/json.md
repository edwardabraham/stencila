# JSON

## Introduction

[JSON (JavaScript Object Notation)](https://www.json.org/) is a lightweight data interchange format widely used for structured data storage and transmission. It is easy for both humans and machines to read and write. JSON's simplicity, flexibility, and compatibility with various programming languages make it a popular choice for APIs, configuration files, and data exchange between applications. 

Its benefits include simplicity, and support for nested data structures, making it a good choice for lossless serialization of Stencila documents for inter-application communication.

## Implementation

Stencila support lossless, bi-directional conversion between Stencila documents and JSON. The `codec-json` Rust crate implements `from_json` and `to_json` methods (and variants of those) for all node types in Stencila Schema, powered by [`serde_json`](https://crates.io/crates/serde_json). 

## Encodings

By default, the encoded JSON is indented but the `--compact` option is supported which produces un-indented, single line JSON.

When the `--standalone` option is used (the default for encoding to files), two properties are added to the JSON encoding of root nodes to improve interoperability:

- a `$schema` property which links to the [JSON Schema](https://json-schema.org) for the node type
- a `@context` property which links to the [JSON-LD](https://json-ld.org) context for the node type

For example,

```json
{
  "$schema": "https://stencila.dev/Article.schema.json",
  "@context": "https://stencila.dev/Article.jsonld",
  "type": "Article",
  ...
```

<!-- prettier-ignore-start -->
<!-- CODEC-DOCS:START -->

## Codec

The codec (en**co**der/**dec**oder) for JSON supports:

- decoding from a file
- decoding from a string
- encoding to a file
- encoding to a string

Support and degree of loss for node types:

| Node type                                                                                                                 | Encoding     | Decoding     | Notes |
| ------------------------------------------------------------------------------------------------------------------------- | ------------ | ------------ | ----- |
| **Works**                                                                                                                 |
| [Article](https://github.com/stencila/stencila/blob/main/docs/reference/schema/works/article.md)                          | 🟢 No loss    | 🟢 No loss    |       |
| [AudioObject](https://github.com/stencila/stencila/blob/main/docs/reference/schema/works/audio_object.md)                 | 🟢 No loss    | 🟢 No loss    |       |
| [Claim](https://github.com/stencila/stencila/blob/main/docs/reference/schema/works/claim.md)                              | 🟢 No loss    | 🟢 No loss    |       |
| [Collection](https://github.com/stencila/stencila/blob/main/docs/reference/schema/works/collection.md)                    | 🟢 No loss    | 🟢 No loss    |       |
| [Comment](https://github.com/stencila/stencila/blob/main/docs/reference/schema/works/comment.md)                          | 🟢 No loss    | 🟢 No loss    |       |
| [CreativeWork](https://github.com/stencila/stencila/blob/main/docs/reference/schema/works/creative_work.md)               | 🟢 No loss    | 🟢 No loss    |       |
| [Directory](https://github.com/stencila/stencila/blob/main/docs/reference/schema/works/directory.md)                      | 🟢 No loss    | 🟢 No loss    |       |
| [Figure](https://github.com/stencila/stencila/blob/main/docs/reference/schema/works/figure.md)                            | 🟢 No loss    | 🟢 No loss    |       |
| [File](https://github.com/stencila/stencila/blob/main/docs/reference/schema/works/file.md)                                | 🟢 No loss    | 🟢 No loss    |       |
| [ImageObject](https://github.com/stencila/stencila/blob/main/docs/reference/schema/works/image_object.md)                 | 🟢 No loss    | 🟢 No loss    |       |
| [MediaObject](https://github.com/stencila/stencila/blob/main/docs/reference/schema/works/media_object.md)                 | 🟢 No loss    | 🟢 No loss    |       |
| [Periodical](https://github.com/stencila/stencila/blob/main/docs/reference/schema/works/periodical.md)                    | 🟢 No loss    | 🟢 No loss    |       |
| [PublicationIssue](https://github.com/stencila/stencila/blob/main/docs/reference/schema/works/publication_issue.md)       | 🟢 No loss    | 🟢 No loss    |       |
| [PublicationVolume](https://github.com/stencila/stencila/blob/main/docs/reference/schema/works/publication_volume.md)     | 🟢 No loss    | 🟢 No loss    |       |
| [Review](https://github.com/stencila/stencila/blob/main/docs/reference/schema/works/review.md)                            | 🟢 No loss    | 🟢 No loss    |       |
| [SoftwareApplication](https://github.com/stencila/stencila/blob/main/docs/reference/schema/works/software_application.md) | 🟢 No loss    | 🟢 No loss    |       |
| [SoftwareSourceCode](https://github.com/stencila/stencila/blob/main/docs/reference/schema/works/software_source_code.md)  | 🟢 No loss    | 🟢 No loss    |       |
| [Table](https://github.com/stencila/stencila/blob/main/docs/reference/schema/works/table.md)                              | 🟢 No loss    | 🟢 No loss    |       |
| [TableCell](https://github.com/stencila/stencila/blob/main/docs/reference/schema/works/table_cell.md)                     | 🟢 No loss    | 🟢 No loss    |       |
| [TableRow](https://github.com/stencila/stencila/blob/main/docs/reference/schema/works/table_row.md)                       | 🟢 No loss    | 🟢 No loss    |       |
| [VideoObject](https://github.com/stencila/stencila/blob/main/docs/reference/schema/works/video_object.md)                 | 🟢 No loss    | 🟢 No loss    |       |
| **Prose**                                                                                                                 |
| [Cite](https://github.com/stencila/stencila/blob/main/docs/reference/schema/prose/cite.md)                                | 🟢 No loss    | 🟢 No loss    |       |
| [CiteGroup](https://github.com/stencila/stencila/blob/main/docs/reference/schema/prose/cite_group.md)                     | 🟢 No loss    | 🟢 No loss    |       |
| [DefinedTerm](https://github.com/stencila/stencila/blob/main/docs/reference/schema/prose/defined_term.md)                 | 🟢 No loss    | 🟢 No loss    |       |
| [Delete](https://github.com/stencila/stencila/blob/main/docs/reference/schema/prose/delete.md)                            | 🟢 No loss    | 🟢 No loss    |       |
| [Emphasis](https://github.com/stencila/stencila/blob/main/docs/reference/schema/prose/emphasis.md)                        | 🟢 No loss    | 🟢 No loss    |       |
| [Heading](https://github.com/stencila/stencila/blob/main/docs/reference/schema/prose/heading.md)                          | 🟢 No loss    | 🟢 No loss    |       |
| [Insert](https://github.com/stencila/stencila/blob/main/docs/reference/schema/prose/insert.md)                            | 🟢 No loss    | 🟢 No loss    |       |
| [Link](https://github.com/stencila/stencila/blob/main/docs/reference/schema/prose/link.md)                                | 🟢 No loss    | 🟢 No loss    |       |
| [List](https://github.com/stencila/stencila/blob/main/docs/reference/schema/prose/list.md)                                | 🟢 No loss    | 🟢 No loss    |       |
| [ListItem](https://github.com/stencila/stencila/blob/main/docs/reference/schema/prose/list_item.md)                       | 🟢 No loss    | 🟢 No loss    |       |
| [Note](https://github.com/stencila/stencila/blob/main/docs/reference/schema/prose/note.md)                                | 🟢 No loss    | 🟢 No loss    |       |
| [Paragraph](https://github.com/stencila/stencila/blob/main/docs/reference/schema/prose/paragraph.md)                      | 🟢 No loss    | 🟢 No loss    |       |
| [Quote](https://github.com/stencila/stencila/blob/main/docs/reference/schema/prose/quote.md)                              | 🟢 No loss    | 🟢 No loss    |       |
| [QuoteBlock](https://github.com/stencila/stencila/blob/main/docs/reference/schema/prose/quote_block.md)                   | 🟢 No loss    | 🟢 No loss    |       |
| [Section](https://github.com/stencila/stencila/blob/main/docs/reference/schema/prose/section.md)                          | 🟢 No loss    | 🟢 No loss    |       |
| [Strikeout](https://github.com/stencila/stencila/blob/main/docs/reference/schema/prose/strikeout.md)                      | 🟢 No loss    | 🟢 No loss    |       |
| [Strong](https://github.com/stencila/stencila/blob/main/docs/reference/schema/prose/strong.md)                            | 🟢 No loss    | 🟢 No loss    |       |
| [Subscript](https://github.com/stencila/stencila/blob/main/docs/reference/schema/prose/subscript.md)                      | 🟢 No loss    | 🟢 No loss    |       |
| [Superscript](https://github.com/stencila/stencila/blob/main/docs/reference/schema/prose/superscript.md)                  | 🟢 No loss    | 🟢 No loss    |       |
| [Text](https://github.com/stencila/stencila/blob/main/docs/reference/schema/prose/text.md)                                | 🟢 No loss    | 🟢 No loss    |       |
| [ThematicBreak](https://github.com/stencila/stencila/blob/main/docs/reference/schema/prose/thematic_break.md)             | 🟢 No loss    | 🟢 No loss    |       |
| [Underline](https://github.com/stencila/stencila/blob/main/docs/reference/schema/prose/underline.md)                      | 🟢 No loss    | 🟢 No loss    |       |
| **Math**                                                                                                                  |
| [MathBlock](https://github.com/stencila/stencila/blob/main/docs/reference/schema/math/math_block.md)                      | 🟢 No loss    | 🟢 No loss    |       |
| [MathFragment](https://github.com/stencila/stencila/blob/main/docs/reference/schema/math/math_fragment.md)                | 🟢 No loss    | 🟢 No loss    |       |
| **Code**                                                                                                                  |
| [CodeBlock](https://github.com/stencila/stencila/blob/main/docs/reference/schema/code/code_block.md)                      | 🟢 No loss    | 🟢 No loss    |       |
| [CodeChunk](https://github.com/stencila/stencila/blob/main/docs/reference/schema/code/code_chunk.md)                      | 🟢 No loss    | 🟢 No loss    |       |
| [CodeError](https://github.com/stencila/stencila/blob/main/docs/reference/schema/code/code_error.md)                      | 🟢 No loss    | 🟢 No loss    |       |
| [CodeExpression](https://github.com/stencila/stencila/blob/main/docs/reference/schema/code/code_expression.md)            | 🟢 No loss    | 🟢 No loss    |       |
| [CodeFragment](https://github.com/stencila/stencila/blob/main/docs/reference/schema/code/code_fragment.md)                | 🟢 No loss    | 🟢 No loss    |       |
| **Data**                                                                                                                  |
| [Array](https://github.com/stencila/stencila/blob/main/docs/reference/schema/data/array.md)                               | 🟢 No loss    | 🟢 No loss    |       |
| [ArrayValidator](https://github.com/stencila/stencila/blob/main/docs/reference/schema/data/array_validator.md)            | 🟢 No loss    | 🟢 No loss    |       |
| [Boolean](https://github.com/stencila/stencila/blob/main/docs/reference/schema/data/boolean.md)                           | 🟢 No loss    | 🟢 No loss    |       |
| [BooleanValidator](https://github.com/stencila/stencila/blob/main/docs/reference/schema/data/boolean_validator.md)        | 🟢 No loss    | 🟢 No loss    |       |
| [ConstantValidator](https://github.com/stencila/stencila/blob/main/docs/reference/schema/data/constant_validator.md)      | 🟢 No loss    | 🟢 No loss    |       |
| [Cord](https://github.com/stencila/stencila/blob/main/docs/reference/schema/data/cord.md)                                 | 🟢 No loss    | 🟢 No loss    |       |
| [Datatable](https://github.com/stencila/stencila/blob/main/docs/reference/schema/data/datatable.md)                       | 🟢 No loss    | 🟢 No loss    |       |
| [DatatableColumn](https://github.com/stencila/stencila/blob/main/docs/reference/schema/data/datatable_column.md)          | 🟢 No loss    | 🟢 No loss    |       |
| [Date](https://github.com/stencila/stencila/blob/main/docs/reference/schema/data/date.md)                                 | 🟢 No loss    | 🟢 No loss    |       |
| [DateTime](https://github.com/stencila/stencila/blob/main/docs/reference/schema/data/date_time.md)                        | 🟢 No loss    | 🟢 No loss    |       |
| [DateTimeValidator](https://github.com/stencila/stencila/blob/main/docs/reference/schema/data/date_time_validator.md)     | 🟢 No loss    | 🟢 No loss    |       |
| [DateValidator](https://github.com/stencila/stencila/blob/main/docs/reference/schema/data/date_validator.md)              | 🟢 No loss    | 🟢 No loss    |       |
| [Duration](https://github.com/stencila/stencila/blob/main/docs/reference/schema/data/duration.md)                         | 🟢 No loss    | 🟢 No loss    |       |
| [DurationValidator](https://github.com/stencila/stencila/blob/main/docs/reference/schema/data/duration_validator.md)      | 🟢 No loss    | 🟢 No loss    |       |
| [EnumValidator](https://github.com/stencila/stencila/blob/main/docs/reference/schema/data/enum_validator.md)              | 🟢 No loss    | 🟢 No loss    |       |
| [Integer](https://github.com/stencila/stencila/blob/main/docs/reference/schema/data/integer.md)                           | 🟢 No loss    | 🟢 No loss    |       |
| [IntegerValidator](https://github.com/stencila/stencila/blob/main/docs/reference/schema/data/integer_validator.md)        | 🟢 No loss    | 🟢 No loss    |       |
| [Null](https://github.com/stencila/stencila/blob/main/docs/reference/schema/data/null.md)                                 | 🟢 No loss    | 🟢 No loss    |       |
| [Number](https://github.com/stencila/stencila/blob/main/docs/reference/schema/data/number.md)                             | 🟢 No loss    | 🟢 No loss    |       |
| [NumberValidator](https://github.com/stencila/stencila/blob/main/docs/reference/schema/data/number_validator.md)          | 🟢 No loss    | 🟢 No loss    |       |
| [Object](https://github.com/stencila/stencila/blob/main/docs/reference/schema/data/object.md)                             | 🟢 No loss    | 🟢 No loss    |       |
| [String](https://github.com/stencila/stencila/blob/main/docs/reference/schema/data/string.md)                             | 🟢 No loss    | 🟢 No loss    |       |
| [StringValidator](https://github.com/stencila/stencila/blob/main/docs/reference/schema/data/string_validator.md)          | 🟢 No loss    | 🟢 No loss    |       |
| [Time](https://github.com/stencila/stencila/blob/main/docs/reference/schema/data/time.md)                                 | 🟢 No loss    | 🟢 No loss    |       |
| [TimeValidator](https://github.com/stencila/stencila/blob/main/docs/reference/schema/data/time_validator.md)              | 🟢 No loss    | 🟢 No loss    |       |
| [Timestamp](https://github.com/stencila/stencila/blob/main/docs/reference/schema/data/timestamp.md)                       | 🟢 No loss    | 🟢 No loss    |       |
| [TimestampValidator](https://github.com/stencila/stencila/blob/main/docs/reference/schema/data/timestamp_validator.md)    | 🟢 No loss    | 🟢 No loss    |       |
| [TupleValidator](https://github.com/stencila/stencila/blob/main/docs/reference/schema/data/tuple_validator.md)            | 🟢 No loss    | 🟢 No loss    |       |
| [UnsignedInteger](https://github.com/stencila/stencila/blob/main/docs/reference/schema/data/unsigned_integer.md)          | 🟢 No loss    | 🟢 No loss    |       |
| **Flow**                                                                                                                  |
| [Button](https://github.com/stencila/stencila/blob/main/docs/reference/schema/flow/button.md)                             | 🟢 No loss    | 🟢 No loss    |       |
| [Call](https://github.com/stencila/stencila/blob/main/docs/reference/schema/flow/call.md)                                 | 🟢 No loss    | 🟢 No loss    |       |
| [CallArgument](https://github.com/stencila/stencila/blob/main/docs/reference/schema/flow/call_argument.md)                | 🟢 No loss    | 🟢 No loss    |       |
| [ExecutionDependant](https://github.com/stencila/stencila/blob/main/docs/reference/schema/flow/execution_dependant.md)    | 🟢 No loss    | 🟢 No loss    |       |
| [ExecutionDependency](https://github.com/stencila/stencila/blob/main/docs/reference/schema/flow/execution_dependency.md)  | 🟢 No loss    | 🟢 No loss    |       |
| [ExecutionDigest](https://github.com/stencila/stencila/blob/main/docs/reference/schema/flow/execution_digest.md)          | 🟢 No loss    | 🟢 No loss    |       |
| [ExecutionTag](https://github.com/stencila/stencila/blob/main/docs/reference/schema/flow/execution_tag.md)                | 🟢 No loss    | 🟢 No loss    |       |
| [For](https://github.com/stencila/stencila/blob/main/docs/reference/schema/flow/for.md)                                   | 🟢 No loss    | 🟢 No loss    |       |
| [Form](https://github.com/stencila/stencila/blob/main/docs/reference/schema/flow/form.md)                                 | 🟢 No loss    | 🟢 No loss    |       |
| [Function](https://github.com/stencila/stencila/blob/main/docs/reference/schema/flow/function.md)                         | 🟢 No loss    | 🟢 No loss    |       |
| [If](https://github.com/stencila/stencila/blob/main/docs/reference/schema/flow/if.md)                                     | 🟢 No loss    | 🟢 No loss    |       |
| [IfClause](https://github.com/stencila/stencila/blob/main/docs/reference/schema/flow/if_clause.md)                        | 🟢 No loss    | 🟢 No loss    |       |
| [Include](https://github.com/stencila/stencila/blob/main/docs/reference/schema/flow/include.md)                           | 🟢 No loss    | 🟢 No loss    |       |
| [Parameter](https://github.com/stencila/stencila/blob/main/docs/reference/schema/flow/parameter.md)                       | 🟢 No loss    | 🟢 No loss    |       |
| [Variable](https://github.com/stencila/stencila/blob/main/docs/reference/schema/flow/variable.md)                         | 🟢 No loss    | 🟢 No loss    |       |
| **Style**                                                                                                                 |
| [Division](https://github.com/stencila/stencila/blob/main/docs/reference/schema/style/division.md)                        | 🟢 No loss    | 🟢 No loss    |       |
| [Span](https://github.com/stencila/stencila/blob/main/docs/reference/schema/style/span.md)                                | 🟢 No loss    | 🟢 No loss    |       |
| **Other**                                                                                                                 |
| [Brand](https://github.com/stencila/stencila/blob/main/docs/reference/schema/other/brand.md)                              | 🟢 No loss    | 🟢 No loss    |       |
| [ContactPoint](https://github.com/stencila/stencila/blob/main/docs/reference/schema/other/contact_point.md)               | 🟢 No loss    | 🟢 No loss    |       |
| [Enumeration](https://github.com/stencila/stencila/blob/main/docs/reference/schema/other/enumeration.md)                  | 🟢 No loss    | 🟢 No loss    |       |
| [Grant](https://github.com/stencila/stencila/blob/main/docs/reference/schema/other/grant.md)                              | 🟢 No loss    | 🟢 No loss    |       |
| [MonetaryGrant](https://github.com/stencila/stencila/blob/main/docs/reference/schema/other/monetary_grant.md)             | 🟢 No loss    | 🟢 No loss    |       |
| [Organization](https://github.com/stencila/stencila/blob/main/docs/reference/schema/other/organization.md)                | 🟢 No loss    | 🟢 No loss    |       |
| [Person](https://github.com/stencila/stencila/blob/main/docs/reference/schema/other/person.md)                            | 🟢 No loss    | 🟢 No loss    |       |
| [PostalAddress](https://github.com/stencila/stencila/blob/main/docs/reference/schema/other/postal_address.md)             | 🟢 No loss    | 🟢 No loss    |       |
| [Product](https://github.com/stencila/stencila/blob/main/docs/reference/schema/other/product.md)                          | 🟢 No loss    | 🟢 No loss    |       |
| [PropertyValue](https://github.com/stencila/stencila/blob/main/docs/reference/schema/other/property_value.md)             | 🟢 No loss    | 🟢 No loss    |       |
| [Thing](https://github.com/stencila/stencila/blob/main/docs/reference/schema/other/thing.md)                              | 🟢 No loss    | 🟢 No loss    |       |

<!-- CODEC-DOCS:STOP -->
<!-- prettier-ignore-end -->
