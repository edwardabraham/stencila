---
title:
- type: Text
  value: Button
---

# Button

**A button.**

**`@id`**: `stencila:Button`

This type is marked as experimental and is likely to change.

## Properties

The `Button` type has these properties:

| Name                  | `@id`                                                                  | Type                                                                                           | Description                                                                                       | Inherited from                                                                      |
| --------------------- | ---------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------- |
| id                    | [`schema:id`](https://schema.org/id)                                   | [`String`](https://stencila.dev/docs/reference/schema/data/string)                             | The identifier for this item                                                                      | [`Entity`](https://stencila.dev/docs/reference/schema/other/entity)                 |
| executionAuto         | `stencila:executionAuto`                                               | [`ExecutionAuto`](https://stencila.dev/docs/reference/schema/flow/execution-auto)              | Under which circumstances the code should be automatically executed.                              | [`Executable`](https://stencila.dev/docs/reference/schema/flow/executable)          |
| compilationDigest     | `stencila:compilationDigest`                                           | [`ExecutionDigest`](https://stencila.dev/docs/reference/schema/flow/execution-digest)          | A digest of the content, semantics and dependencies of the node.                                  | [`Executable`](https://stencila.dev/docs/reference/schema/flow/executable)          |
| executionDigest       | `stencila:executionDigest`                                             | [`ExecutionDigest`](https://stencila.dev/docs/reference/schema/flow/execution-digest)          | The `compileDigest` of the node when it was last executed.                                        | [`Executable`](https://stencila.dev/docs/reference/schema/flow/executable)          |
| executionDependencies | `stencila:executionDependencies`                                       | [`ExecutionDependency`](https://stencila.dev/docs/reference/schema/flow/execution-dependency)* | The upstream dependencies of this node.                                                           | [`Executable`](https://stencila.dev/docs/reference/schema/flow/executable)          |
| executionDependants   | `stencila:executionDependants`                                         | [`ExecutionDependant`](https://stencila.dev/docs/reference/schema/flow/execution-dependant)*   | The downstream dependants of this node.                                                           | [`Executable`](https://stencila.dev/docs/reference/schema/flow/executable)          |
| executionTags         | `stencila:executionTags`                                               | [`ExecutionTag`](https://stencila.dev/docs/reference/schema/flow/execution-tag)*               | Tags in the code which affect its execution                                                       | [`Executable`](https://stencila.dev/docs/reference/schema/flow/executable)          |
| executionCount        | `stencila:executionCount`                                              | [`Integer`](https://stencila.dev/docs/reference/schema/data/integer)                           | A count of the number of times that the node has been executed.                                   | [`Executable`](https://stencila.dev/docs/reference/schema/flow/executable)          |
| executionRequired     | `stencila:executionRequired`                                           | [`ExecutionRequired`](https://stencila.dev/docs/reference/schema/flow/execution-required)      | Whether, and why, the code requires execution or re-execution.                                    | [`Executable`](https://stencila.dev/docs/reference/schema/flow/executable)          |
| executionKernel       | `stencila:executionKernel`                                             | [`String`](https://stencila.dev/docs/reference/schema/data/string)                             | The id of the kernel that the node was last executed in.                                          | [`Executable`](https://stencila.dev/docs/reference/schema/flow/executable)          |
| executionStatus       | `stencila:executionStatus`                                             | [`ExecutionStatus`](https://stencila.dev/docs/reference/schema/flow/execution-status)          | Status of the most recent, including any current, execution.                                      | [`Executable`](https://stencila.dev/docs/reference/schema/flow/executable)          |
| executionEnded        | `stencila:executionEnded`                                              | [`Timestamp`](https://stencila.dev/docs/reference/schema/data/timestamp)                       | The timestamp when the last execution ended.                                                      | [`Executable`](https://stencila.dev/docs/reference/schema/flow/executable)          |
| executionDuration     | `stencila:executionDuration`                                           | [`Duration`](https://stencila.dev/docs/reference/schema/data/duration)                         | Duration of the last execution.                                                                   | [`Executable`](https://stencila.dev/docs/reference/schema/flow/executable)          |
| errors                | `stencila:errors`                                                      | [`CodeError`](https://stencila.dev/docs/reference/schema/code/code-error)*                     | Errors when compiling (e.g. syntax errors) or executing the node.                                 | [`Executable`](https://stencila.dev/docs/reference/schema/flow/executable)          |
| code                  | `stencila:code`                                                        | [`Cord`](https://stencila.dev/docs/reference/schema/data/cord)                                 | The code.                                                                                         | [`CodeExecutable`](https://stencila.dev/docs/reference/schema/code/code-executable) |
| programmingLanguage   | [`schema:programmingLanguage`](https://schema.org/programmingLanguage) | [`String`](https://stencila.dev/docs/reference/schema/data/string)                             | The programming language of the code.                                                             | [`CodeExecutable`](https://stencila.dev/docs/reference/schema/code/code-executable) |
| guessLanguage         | `stencila:guessLanguage`                                               | [`Boolean`](https://stencila.dev/docs/reference/schema/data/boolean)                           | Whether the programming language of the code should be guessed based on syntax and variables used | [`CodeExecutable`](https://stencila.dev/docs/reference/schema/code/code-executable) |
| name                  | [`schema:name`](https://schema.org/name)                               | [`String`](https://stencila.dev/docs/reference/schema/data/string)                             | The name of the variable associated with the button.                                              | [`Button`](https://stencila.dev/docs/reference/schema/flow/button)                  |
| label                 | `stencila:label`                                                       | [`String`](https://stencila.dev/docs/reference/schema/data/string)                             | A label for the button                                                                            | [`Button`](https://stencila.dev/docs/reference/schema/flow/button)                  |
| isDisabled            | `stencila:isDisabled`                                                  | [`Boolean`](https://stencila.dev/docs/reference/schema/data/boolean)                           | Whether the button is currently disabled                                                          | [`Button`](https://stencila.dev/docs/reference/schema/flow/button)                  |

## Related

The `Button` type is related to these types:

- Parents: [`CodeExecutable`](https://stencila.dev/docs/reference/schema/code/code-executable)
- Children: none

## Formats

The `Button` type can be encoded (serialized) to, and/or decoded (deserialized) from, these formats:

| Format                                                           | Encoding       | Decoding     | Status                 | Notes                     |
| ---------------------------------------------------------------- | -------------- | ------------ | ---------------------- | ------------------------- |
| [HTML](https://stencila.dev/docs/reference/formats/{name})       | 🔷 Low loss     |              | 🚧 Under development    | Encoded to tag `<button>` |
| [Markdown](https://stencila.dev/docs/reference/formats/{name})   | 🟥 High loss    |              | 🚧 Under development    |                           |
| [Plain text](https://stencila.dev/docs/reference/formats/{name}) | 🟥 High loss    |              | 🟥 Alpha                |                           |
| [JSON](https://stencila.dev/docs/reference/formats/{name})       | 🟢 No loss      | 🟢 No loss    | 🟢 Stable               |                           |
| [JSON5](https://stencila.dev/docs/reference/formats/{name})      | 🟢 No loss      | 🟢 No loss    | 🟢 Stable               |                           |
| [YAML](https://stencila.dev/docs/reference/formats/{name})       | 🟢 No loss      | 🟢 No loss    | 🟢 Stable               |                           |
| [Debug](https://stencila.dev/docs/reference/formats/{name})      | 🔷 Low loss     |              | 🟢 Stable               |                           |

## Bindings

The `Button` type is represented in these bindings:

- [JSON-LD](https://stencila.dev/Button.jsonld)
- [JSON Schema](https://stencila.dev/Button.schema.json)
- Python class [`Button`](https://github.com/stencila/stencila/blob/main/python/stencila/types/button.py)
- Rust struct [`Button`](https://github.com/stencila/stencila/blob/main/rust/schema/src/types/button.rs)
- TypeScript class [`Button`](https://github.com/stencila/stencila/blob/main/typescript/src/types/Button.ts)

## Source

This documentation was generated from [`Button.yaml`](https://github.com/stencila/stencila/blob/main/schema/Button.yaml) by [`docs.rs`](https://github.com/stencila/stencila/blob/main/rust/schema-gen/src/docs.rs).