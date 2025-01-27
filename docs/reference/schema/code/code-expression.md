# Code Expression

**An executable programming code expression.**

Note that `CodeExpression` nodes lack the `executionPure` property that `CodeChunk` nodes have because they should be side-effect free.

**`@id`**: `stencila:CodeExpression`

## Properties

The `CodeExpression` type has these properties:

| Name                  | Aliases                                                                                                         | `@id`                                                                  | Type                                                                                                                        | Description                                                          | Inherited from                                                                                                   |
| --------------------- | --------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------- |
| id                    | -                                                                                                               | [`schema:id`](https://schema.org/id)                                   | [`String`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/data/string.md)                             | The identifier for this item.                                        | [`Entity`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/other/entity.md)                 |
| autoExec              | auto, auto-exec, auto_exec                                                                                      | `stencila:autoExec`                                                    | [`AutomaticExecution`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/flow/automatic-execution.md)    | Under which circumstances the code should be automatically executed. | [`Executable`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/flow/executable.md)          |
| compilationDigest     | compilation-digest, compilation_digest                                                                          | `stencila:compilationDigest`                                           | [`ExecutionDigest`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/flow/execution-digest.md)          | A digest of the content, semantics and dependencies of the node.     | [`Executable`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/flow/executable.md)          |
| executionDigest       | execution-digest, execution_digest                                                                              | `stencila:executionDigest`                                             | [`ExecutionDigest`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/flow/execution-digest.md)          | The `compileDigest` of the node when it was last executed.           | [`Executable`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/flow/executable.md)          |
| executionDependencies | execution-dependencies, execution_dependencies, executionDependency, execution-dependency, execution_dependency | `stencila:executionDependencies`                                       | [`ExecutionDependency`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/flow/execution-dependency.md)* | The upstream dependencies of this node.                              | [`Executable`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/flow/executable.md)          |
| executionDependants   | execution-dependants, execution_dependants, executionDependant, execution-dependant, execution_dependant        | `stencila:executionDependants`                                         | [`ExecutionDependant`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/flow/execution-dependant.md)*   | The downstream dependants of this node.                              | [`Executable`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/flow/executable.md)          |
| executionTags         | execution-tags, execution_tags, executionTag, execution-tag, execution_tag                                      | `stencila:executionTags`                                               | [`ExecutionTag`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/flow/execution-tag.md)*               | Tags in the code which affect its execution.                         | [`Executable`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/flow/executable.md)          |
| executionCount        | execution-count, execution_count                                                                                | `stencila:executionCount`                                              | [`Integer`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/data/integer.md)                           | A count of the number of times that the node has been executed.      | [`Executable`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/flow/executable.md)          |
| executionRequired     | execution-required, execution_required                                                                          | `stencila:executionRequired`                                           | [`ExecutionRequired`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/flow/execution-required.md)      | Whether, and why, the code requires execution or re-execution.       | [`Executable`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/flow/executable.md)          |
| executionKernel       | execution-kernel, execution_kernel                                                                              | `stencila:executionKernel`                                             | [`String`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/data/string.md)                             | The id of the kernel that the node was last executed in.             | [`Executable`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/flow/executable.md)          |
| executionStatus       | execution-status, execution_status                                                                              | `stencila:executionStatus`                                             | [`ExecutionStatus`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/flow/execution-status.md)          | Status of the most recent, including any current, execution.         | [`Executable`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/flow/executable.md)          |
| executionEnded        | execution-ended, execution_ended                                                                                | `stencila:executionEnded`                                              | [`Timestamp`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/data/timestamp.md)                       | The timestamp when the last execution ended.                         | [`Executable`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/flow/executable.md)          |
| executionDuration     | execution-duration, execution_duration                                                                          | `stencila:executionDuration`                                           | [`Duration`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/data/duration.md)                         | Duration of the last execution.                                      | [`Executable`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/flow/executable.md)          |
| errors                | error                                                                                                           | `stencila:errors`                                                      | [`CodeError`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/code/code-error.md)*                     | Errors when compiling (e.g. syntax errors) or executing the node.    | [`Executable`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/flow/executable.md)          |
| code                  | -                                                                                                               | `stencila:code`                                                        | [`Cord`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/data/cord.md)                                 | The code.                                                            | [`CodeExecutable`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/code/code-executable.md) |
| programmingLanguage   | programming-language, programming_language                                                                      | [`schema:programmingLanguage`](https://schema.org/programmingLanguage) | [`String`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/data/string.md)                             | The programming language of the code.                                | [`CodeExecutable`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/code/code-executable.md) |
| output                | -                                                                                                               | `stencila:output`                                                      | [`Node`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/other/node.md)                                | The value of the expression when it was last evaluated.              | -                                                                                                                |

## Related

The `CodeExpression` type is related to these types:

- Parents: [`CodeExecutable`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/code/code-executable.md)
- Children: none

## Formats

The `CodeExpression` type can be encoded (serialized) to, and/or decoded (deserialized) from, these formats:

| Format                                                                                        | Encoding         | Decoding      | Status                 | Notes                                                                                                  |
| --------------------------------------------------------------------------------------------- | ---------------- | ------------- | ---------------------- | ------------------------------------------------------------------------------------------------------ |
| [HTML](https://github.com/stencila/stencila/blob/main/docs/reference/formats/html.md)         | 🔷 Low loss       |               | 🚧 Under development    | Encoded to tag [`<span>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/span)              |
| [JATS](https://github.com/stencila/stencila/blob/main/docs/reference/formats/jats.md)         | 🔷 Low loss       | 🔷 Low loss    | 🚧 Under development    | Encoded to tag [`<code>`](https://jats.nlm.nih.gov/articleauthoring/tag-library/1.3/element/code.html) |
| [Markdown](https://github.com/stencila/stencila/blob/main/docs/reference/formats/markdown.md) | 🔷 Low loss       |               | 🚧 Under development    | Encoded using special function                                                                         |
| [Plain text](https://github.com/stencila/stencila/blob/main/docs/reference/formats/text.md)   | ⚠️ High loss     |               | ⚠️ Alpha               |                                                                                                        |
| [JSON](https://github.com/stencila/stencila/blob/main/docs/reference/formats/json.md)         | 🟢 No loss        | 🟢 No loss     | 🟢 Stable               |                                                                                                        |
| [JSON5](https://github.com/stencila/stencila/blob/main/docs/reference/formats/json5.md)       | 🟢 No loss        | 🟢 No loss     | 🟢 Stable               |                                                                                                        |
| [YAML](https://github.com/stencila/stencila/blob/main/docs/reference/formats/yaml.md)         | 🟢 No loss        | 🟢 No loss     | 🟢 Stable               |                                                                                                        |
| [Debug](https://github.com/stencila/stencila/blob/main/docs/reference/formats/debug.md)       | 🔷 Low loss       |               | 🟢 Stable               |                                                                                                        |

## Bindings

The `CodeExpression` type is represented in these bindings:

- [JSON-LD](https://stencila.dev/CodeExpression.jsonld)
- [JSON Schema](https://stencila.dev/CodeExpression.schema.json)
- Python class [`CodeExpression`](https://github.com/stencila/stencila/blob/main/python/python/stencila/types/code_expression.py)
- Rust struct [`CodeExpression`](https://github.com/stencila/stencila/blob/main/rust/schema/src/types/code_expression.rs)
- TypeScript class [`CodeExpression`](https://github.com/stencila/stencila/blob/main/typescript/src/types/CodeExpression.ts)

## Testing

During property-based (a.k.a generative) testing, the properties of the `CodeExpression` type are generated using the following strategies for each complexity level (see the [`proptest` book](https://proptest-rs.github.io/proptest/) for an explanation of the Rust strategy expressions). Any optional properties that are not in this table are set to `None`.

| Property              | Complexity | Description                                                                      | Strategy                                        |
| --------------------- | ---------- | -------------------------------------------------------------------------------- | ----------------------------------------------- |
| `code`                | Min+       | Generate a simple fixed string of code.                                          | `Cord::new("code")`                             |
|                       | Low+       | Generate a random string of up to 10 alphanumeric & whitespace characters.       | `r"[a-zA-Z0-9 \t\n]{1,10}".prop_map(Cord::new)` |
|                       | High+      | Generate a random string of up to 100 characters (excluding control characters). | `r"[^\p{C}]{1,100}".prop_map(Cord::new)`        |
|                       | Max        | Generate an arbitrary string.                                                    | `String::arbitrary().prop_map(Cord::new)`       |
| `programmingLanguage` | Min+       | Generate a simple fixed string.                                                  | `Some(String::from("lang"))`                    |
|                       | Low+       | Generate one of the well known programming language short names.                 | `option::of(r"(cpp)\|(js)\|(py)\|(r)\|(ts)")`   |
|                       | High+      | Generate a random string of up to 10 alphanumeric characters.                    | `option::of(r"[a-zA-Z0-9]{1,10}")`              |
|                       | Max        | Generate an arbitrary string.                                                    | `option::of(String::arbitrary())`               |

## Source

This documentation was generated from [`CodeExpression.yaml`](https://github.com/stencila/stencila/blob/main/schema/CodeExpression.yaml) by [`docs.rs`](https://github.com/stencila/stencila/blob/main/rust/schema-gen/src/docs.rs).