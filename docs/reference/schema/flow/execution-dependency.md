# Execution Dependency

**An upstream execution dependency of a node.**

**`@id`**: `stencila:ExecutionDependency`

## Properties

The `ExecutionDependency` type has these properties:

| Name               | Aliases                                  | `@id`                                | Type                                                                                                                                        | Description                                             | Inherited from                                                                                   |
| ------------------ | ---------------------------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------- | ------------------------------------------------------------------------------------------------ |
| id                 | -                                        | [`schema:id`](https://schema.org/id) | [`String`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/data/string.md)                                             | The identifier for this item.                           | [`Entity`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/other/entity.md) |
| dependencyRelation | dependency-relation, dependency_relation | `stencila:dependencyRelation`        | [`ExecutionDependencyRelation`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/flow/execution-dependency-relation.md) | The relation to the dependency                          | -                                                                                                |
| dependencyNode     | dependency-node, dependency_node         | `stencila:dependencyNode`            | [`ExecutionDependencyNode`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/flow/execution-dependency-node.md)         | The node that is the dependency                         | -                                                                                                |
| codeLocation       | code-location, code_location             | `stencila:codeLocation`              | [`Integer`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/data/integer.md)*                                          | The location that the dependency is defined within code | -                                                                                                |

## Related

The `ExecutionDependency` type is related to these types:

- Parents: [`Entity`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/other/entity.md)
- Children: none

## Formats

The `ExecutionDependency` type can be encoded (serialized) to, and/or decoded (deserialized) from, these formats:

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

The `ExecutionDependency` type is represented in these bindings:

- [JSON-LD](https://stencila.dev/ExecutionDependency.jsonld)
- [JSON Schema](https://stencila.dev/ExecutionDependency.schema.json)
- Python class [`ExecutionDependency`](https://github.com/stencila/stencila/blob/main/python/python/stencila/types/execution_dependency.py)
- Rust struct [`ExecutionDependency`](https://github.com/stencila/stencila/blob/main/rust/schema/src/types/execution_dependency.rs)
- TypeScript class [`ExecutionDependency`](https://github.com/stencila/stencila/blob/main/typescript/src/types/ExecutionDependency.ts)

## Source

This documentation was generated from [`ExecutionDependency.yaml`](https://github.com/stencila/stencila/blob/main/schema/ExecutionDependency.yaml) by [`docs.rs`](https://github.com/stencila/stencila/blob/main/rust/schema-gen/src/docs.rs).