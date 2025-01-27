# Math

**Abstract base type for a mathematical variable or equation.**

This is a base type for `MathFragment` and `MathBlock` and should not
normally be instantiated.
This type has a similar structure and purpose to `CodeStatic` which is a base type
for `CodeFragment`, `CodeBlock` etc.


**`@id`**: `stencila:Math`

## Properties

The `Math` type has these properties:

| Name          | Aliases                        | `@id`                                | Type                                                                                                               | Description                                                    | Inherited from                                                                                   |
| ------------- | ------------------------------ | ------------------------------------ | ------------------------------------------------------------------------------------------------------------------ | -------------------------------------------------------------- | ------------------------------------------------------------------------------------------------ |
| id            | -                              | [`schema:id`](https://schema.org/id) | [`String`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/data/string.md)                    | The identifier for this item.                                  | [`Entity`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/other/entity.md) |
| mathLanguage  | math-language, math_language   | `stencila:mathLanguage`              | [`String`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/data/string.md)                    | The language used for the equation e.g tex, mathml, asciimath. | -                                                                                                |
| code          | -                              | `stencila:code`                      | [`Cord`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/data/cord.md)                        | The code of the equation in the `mathLanguage`.                | -                                                                                                |
| compileDigest | compile-digest, compile_digest | `stencila:compileDigest`             | [`ExecutionDigest`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/flow/execution-digest.md) | A digest of the `code` and `mathLanguage`.                     | -                                                                                                |
| errors        | error                          | `stencila:errors`                    | [`String`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/data/string.md)*                   | Errors that occurred when parsing the math equation.           | -                                                                                                |
| mathml        | -                              | `stencila:mathml`                    | [`String`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/data/string.md)                    | The MathML transpiled from the `code`.                         | -                                                                                                |

## Related

The `Math` type is related to these types:

- Parents: [`Entity`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/other/entity.md)
- Children: [`MathBlock`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/math/math-block.md), [`MathFragment`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/math/math-fragment.md)

## Bindings

The `Math` type is represented in these bindings:

- [JSON-LD](https://stencila.dev/Math.jsonld)
- [JSON Schema](https://stencila.dev/Math.schema.json)
- Python class [`Math`](https://github.com/stencila/stencila/blob/main/python/python/stencila/types/math.py)
- Rust struct [`Math`](https://github.com/stencila/stencila/blob/main/rust/schema/src/types/math.rs)
- TypeScript class [`Math`](https://github.com/stencila/stencila/blob/main/typescript/src/types/Math.ts)

## Source

This documentation was generated from [`Math.yaml`](https://github.com/stencila/stencila/blob/main/schema/Math.yaml) by [`docs.rs`](https://github.com/stencila/stencila/blob/main/rust/schema-gen/src/docs.rs).