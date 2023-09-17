---
title:
- type: Text
  value: CallArgument
---

# Call Argument

**The value of a `Parameter` to call a document with**

A `CallArgument` extends `Parameter` by also has some of the same properties as `CodeExecutable` allowing
an expression to be used to set the value of the parameter.


**`@id`**: `stencila:CallArgument`

This type is marked as experimental and is likely to change.

## Properties

The `CallArgument` type has these properties:

| Name                  | `@id`                                                                  | Type                                                                                           | Description                                                                                            | Inherited from                                                                  |
| --------------------- | ---------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------- |
| id                    | [`schema:id`](https://schema.org/id)                                   | [`String`](https://stencila.dev/docs/reference/schema/data/string)                             | The identifier for this item                                                                           | [`Entity`](https://stencila.dev/docs/reference/schema/other/entity)             |
| executionAuto         | `stencila:executionAuto`                                               | [`ExecutionAuto`](https://stencila.dev/docs/reference/schema/flow/execution-auto)              | Under which circumstances the code should be automatically executed.                                   | [`Executable`](https://stencila.dev/docs/reference/schema/flow/executable)      |
| compilationDigest     | `stencila:compilationDigest`                                           | [`ExecutionDigest`](https://stencila.dev/docs/reference/schema/flow/execution-digest)          | A digest of the content, semantics and dependencies of the node.                                       | [`Executable`](https://stencila.dev/docs/reference/schema/flow/executable)      |
| executionDigest       | `stencila:executionDigest`                                             | [`ExecutionDigest`](https://stencila.dev/docs/reference/schema/flow/execution-digest)          | The `compileDigest` of the node when it was last executed.                                             | [`Executable`](https://stencila.dev/docs/reference/schema/flow/executable)      |
| executionDependencies | `stencila:executionDependencies`                                       | [`ExecutionDependency`](https://stencila.dev/docs/reference/schema/flow/execution-dependency)* | The upstream dependencies of this node.                                                                | [`Executable`](https://stencila.dev/docs/reference/schema/flow/executable)      |
| executionDependants   | `stencila:executionDependants`                                         | [`ExecutionDependant`](https://stencila.dev/docs/reference/schema/flow/execution-dependant)*   | The downstream dependants of this node.                                                                | [`Executable`](https://stencila.dev/docs/reference/schema/flow/executable)      |
| executionTags         | `stencila:executionTags`                                               | [`ExecutionTag`](https://stencila.dev/docs/reference/schema/flow/execution-tag)*               | Tags in the code which affect its execution                                                            | [`Executable`](https://stencila.dev/docs/reference/schema/flow/executable)      |
| executionCount        | `stencila:executionCount`                                              | [`Integer`](https://stencila.dev/docs/reference/schema/data/integer)                           | A count of the number of times that the node has been executed.                                        | [`Executable`](https://stencila.dev/docs/reference/schema/flow/executable)      |
| executionRequired     | `stencila:executionRequired`                                           | [`ExecutionRequired`](https://stencila.dev/docs/reference/schema/flow/execution-required)      | Whether, and why, the code requires execution or re-execution.                                         | [`Executable`](https://stencila.dev/docs/reference/schema/flow/executable)      |
| executionKernel       | `stencila:executionKernel`                                             | [`String`](https://stencila.dev/docs/reference/schema/data/string)                             | The id of the kernel that the node was last executed in.                                               | [`Executable`](https://stencila.dev/docs/reference/schema/flow/executable)      |
| executionStatus       | `stencila:executionStatus`                                             | [`ExecutionStatus`](https://stencila.dev/docs/reference/schema/flow/execution-status)          | Status of the most recent, including any current, execution.                                           | [`Executable`](https://stencila.dev/docs/reference/schema/flow/executable)      |
| executionEnded        | `stencila:executionEnded`                                              | [`Timestamp`](https://stencila.dev/docs/reference/schema/data/timestamp)                       | The timestamp when the last execution ended.                                                           | [`Executable`](https://stencila.dev/docs/reference/schema/flow/executable)      |
| executionDuration     | `stencila:executionDuration`                                           | [`Duration`](https://stencila.dev/docs/reference/schema/data/duration)                         | Duration of the last execution.                                                                        | [`Executable`](https://stencila.dev/docs/reference/schema/flow/executable)      |
| errors                | `stencila:errors`                                                      | [`CodeError`](https://stencila.dev/docs/reference/schema/code/code-error)*                     | Errors when compiling (e.g. syntax errors) or executing the node.                                      | [`Executable`](https://stencila.dev/docs/reference/schema/flow/executable)      |
| name                  | [`schema:name`](https://schema.org/name)                               | [`String`](https://stencila.dev/docs/reference/schema/data/string)                             | The name of the parameter.                                                                             | [`Parameter`](https://stencila.dev/docs/reference/schema/flow/parameter)        |
| label                 | `stencila:label`                                                       | [`String`](https://stencila.dev/docs/reference/schema/data/string)                             | A short label for the parameter.                                                                       | [`Parameter`](https://stencila.dev/docs/reference/schema/flow/parameter)        |
| value                 | [`schema:value`](https://schema.org/value)                             | [`Node`](https://stencila.dev/docs/reference/schema/other/node)                                | The current value of the parameter.                                                                    | [`Parameter`](https://stencila.dev/docs/reference/schema/flow/parameter)        |
| default               | [`schema:defaultValue`](https://schema.org/defaultValue)               | [`Node`](https://stencila.dev/docs/reference/schema/other/node)                                | The default value of the parameter.                                                                    | [`Parameter`](https://stencila.dev/docs/reference/schema/flow/parameter)        |
| validator             | `stencila:validator`                                                   | [`Validator`](https://stencila.dev/docs/reference/schema/data/validator)                       | The validator that the value is validated against.                                                     | [`Parameter`](https://stencila.dev/docs/reference/schema/flow/parameter)        |
| hidden                | `stencila:hidden`                                                      | [`Boolean`](https://stencila.dev/docs/reference/schema/data/boolean)                           | Whether the parameter should be hidden.                                                                | [`Parameter`](https://stencila.dev/docs/reference/schema/flow/parameter)        |
| derivedFrom           | `stencila:derivedFrom`                                                 | [`String`](https://stencila.dev/docs/reference/schema/data/string)                             | The dotted path to the object (e.g. a database table column) that the parameter should be derived from | [`Parameter`](https://stencila.dev/docs/reference/schema/flow/parameter)        |
| code                  | `stencila:code`                                                        | [`String`](https://stencila.dev/docs/reference/schema/data/string)                             | The code to be evaluated for the parameter.                                                            | [`CallArgument`](https://stencila.dev/docs/reference/schema/flow/call-argument) |
| programmingLanguage   | [`schema:programmingLanguage`](https://schema.org/programmingLanguage) | [`String`](https://stencila.dev/docs/reference/schema/data/string)                             | The programming language of the code.                                                                  | [`CallArgument`](https://stencila.dev/docs/reference/schema/flow/call-argument) |
| guessLanguage         | `stencila:guessLanguage`                                               | [`Boolean`](https://stencila.dev/docs/reference/schema/data/boolean)                           | Whether the programming language of the code should be guessed based on syntax and variables used      | [`CallArgument`](https://stencila.dev/docs/reference/schema/flow/call-argument) |

## Related

The `CallArgument` type is related to these types:

- Parents: [`Parameter`](https://stencila.dev/docs/reference/schema/flow/parameter)
- Children: none

## Formats

The `CallArgument` type can be encoded (serialized) to, and/or decoded (deserialized) from, these formats:

| Format                                                           | Encoding       | Decoding     | Status                 | Notes |
| ---------------------------------------------------------------- | -------------- | ------------ | ---------------------- | ----- |
| [HTML](https://stencila.dev/docs/reference/formats/{name})       | 🔷 Low loss     |              | 🚧 Under development    |       |
| [JATS](https://stencila.dev/docs/reference/formats/{name})       | 🔷 Low loss     |              | 🚧 Under development    |       |
| [Markdown](https://stencila.dev/docs/reference/formats/{name})   | 🟥 High loss    |              | 🚧 Under development    |       |
| [Plain text](https://stencila.dev/docs/reference/formats/{name}) | 🟥 High loss    |              | 🟥 Alpha                |       |
| [JSON](https://stencila.dev/docs/reference/formats/{name})       | 🟢 No loss      | 🟢 No loss    | 🟢 Stable               |       |
| [JSON5](https://stencila.dev/docs/reference/formats/{name})      | 🟢 No loss      | 🟢 No loss    | 🟢 Stable               |       |
| [YAML](https://stencila.dev/docs/reference/formats/{name})       | 🟢 No loss      | 🟢 No loss    | 🟢 Stable               |       |
| [Debug](https://stencila.dev/docs/reference/formats/{name})      | 🔷 Low loss     |              | 🟢 Stable               |       |

## Bindings

The `CallArgument` type is represented in these bindings:

- [JSON-LD](https://stencila.dev/CallArgument.jsonld)
- [JSON Schema](https://stencila.dev/CallArgument.schema.json)
- Python class [`CallArgument`](https://github.com/stencila/stencila/blob/main/python/stencila/types/call_argument.py)
- Rust struct [`CallArgument`](https://github.com/stencila/stencila/blob/main/rust/schema/src/types/call_argument.rs)
- TypeScript class [`CallArgument`](https://github.com/stencila/stencila/blob/main/typescript/src/types/CallArgument.ts)

## Source

This documentation was generated from [`CallArgument.yaml`](https://github.com/stencila/stencila/blob/main/schema/CallArgument.yaml) by [`docs.rs`](https://github.com/stencila/stencila/blob/main/rust/schema-gen/src/docs.rs).