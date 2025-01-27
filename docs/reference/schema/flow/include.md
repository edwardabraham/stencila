# Include

**Include content from an external source (e.g. file, URL).**

**`@id`**: `stencila:Include`

This type is marked as unstable and is subject to change.

## Properties

The `Include` type has these properties:

| Name                  | Aliases                                                                                                         | `@id`                                                        | Type                                                                                                                        | Description                                                          | Inherited from                                                                                          |
| --------------------- | --------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------ | --------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------- |
| id                    | -                                                                                                               | [`schema:id`](https://schema.org/id)                         | [`String`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/data/string.md)                             | The identifier for this item.                                        | [`Entity`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/other/entity.md)        |
| autoExec              | auto, auto-exec, auto_exec                                                                                      | `stencila:autoExec`                                          | [`AutomaticExecution`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/flow/automatic-execution.md)    | Under which circumstances the code should be automatically executed. | [`Executable`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/flow/executable.md) |
| compilationDigest     | compilation-digest, compilation_digest                                                                          | `stencila:compilationDigest`                                 | [`ExecutionDigest`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/flow/execution-digest.md)          | A digest of the content, semantics and dependencies of the node.     | [`Executable`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/flow/executable.md) |
| executionDigest       | execution-digest, execution_digest                                                                              | `stencila:executionDigest`                                   | [`ExecutionDigest`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/flow/execution-digest.md)          | The `compileDigest` of the node when it was last executed.           | [`Executable`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/flow/executable.md) |
| executionDependencies | execution-dependencies, execution_dependencies, executionDependency, execution-dependency, execution_dependency | `stencila:executionDependencies`                             | [`ExecutionDependency`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/flow/execution-dependency.md)* | The upstream dependencies of this node.                              | [`Executable`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/flow/executable.md) |
| executionDependants   | execution-dependants, execution_dependants, executionDependant, execution-dependant, execution_dependant        | `stencila:executionDependants`                               | [`ExecutionDependant`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/flow/execution-dependant.md)*   | The downstream dependants of this node.                              | [`Executable`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/flow/executable.md) |
| executionTags         | execution-tags, execution_tags, executionTag, execution-tag, execution_tag                                      | `stencila:executionTags`                                     | [`ExecutionTag`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/flow/execution-tag.md)*               | Tags in the code which affect its execution.                         | [`Executable`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/flow/executable.md) |
| executionCount        | execution-count, execution_count                                                                                | `stencila:executionCount`                                    | [`Integer`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/data/integer.md)                           | A count of the number of times that the node has been executed.      | [`Executable`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/flow/executable.md) |
| executionRequired     | execution-required, execution_required                                                                          | `stencila:executionRequired`                                 | [`ExecutionRequired`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/flow/execution-required.md)      | Whether, and why, the code requires execution or re-execution.       | [`Executable`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/flow/executable.md) |
| executionKernel       | execution-kernel, execution_kernel                                                                              | `stencila:executionKernel`                                   | [`String`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/data/string.md)                             | The id of the kernel that the node was last executed in.             | [`Executable`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/flow/executable.md) |
| executionStatus       | execution-status, execution_status                                                                              | `stencila:executionStatus`                                   | [`ExecutionStatus`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/flow/execution-status.md)          | Status of the most recent, including any current, execution.         | [`Executable`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/flow/executable.md) |
| executionEnded        | execution-ended, execution_ended                                                                                | `stencila:executionEnded`                                    | [`Timestamp`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/data/timestamp.md)                       | The timestamp when the last execution ended.                         | [`Executable`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/flow/executable.md) |
| executionDuration     | execution-duration, execution_duration                                                                          | `stencila:executionDuration`                                 | [`Duration`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/data/duration.md)                         | Duration of the last execution.                                      | [`Executable`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/flow/executable.md) |
| errors                | error                                                                                                           | `stencila:errors`                                            | [`CodeError`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/code/code-error.md)*                     | Errors when compiling (e.g. syntax errors) or executing the node.    | [`Executable`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/flow/executable.md) |
| source                | -                                                                                                               | `stencila:source`                                            | [`String`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/data/string.md)                             | The external source of the content, a file path or URL.              | -                                                                                                       |
| mediaType             | encodingFormat, media-type, media_type                                                                          | [`schema:encodingFormat`](https://schema.org/encodingFormat) | [`String`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/data/string.md)                             | Media type of the source content.                                    | -                                                                                                       |
| select                | -                                                                                                               | `stencila:select`                                            | [`String`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/data/string.md)                             | A query to select a subset of content from the source                | -                                                                                                       |
| content               | -                                                                                                               | `stencila:content`                                           | [`Block`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/prose/block.md)*                             | The structured content decoded from the source.                      | -                                                                                                       |

## Related

The `Include` type is related to these types:

- Parents: [`Executable`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/flow/executable.md)
- Children: [`Call`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/flow/call.md)

## Formats

The `Include` type can be encoded (serialized) to, and/or decoded (deserialized) from, these formats:

| Format                                                                                        | Encoding         | Decoding     | Status                 | Notes                          |
| --------------------------------------------------------------------------------------------- | ---------------- | ------------ | ---------------------- | ------------------------------ |
| [HTML](https://github.com/stencila/stencila/blob/main/docs/reference/formats/html.md)         | 🔷 Low loss       |              | 🚧 Under development    |                                |
| [JATS](https://github.com/stencila/stencila/blob/main/docs/reference/formats/jats.md)         |                  |              | 🚧 Under development    |                                |
| [Markdown](https://github.com/stencila/stencila/blob/main/docs/reference/formats/markdown.md) | ⚠️ High loss     |              | 🚧 Under development    | Encoded using special function |
| [Plain text](https://github.com/stencila/stencila/blob/main/docs/reference/formats/text.md)   | ⚠️ High loss     |              | ⚠️ Alpha               |                                |
| [JSON](https://github.com/stencila/stencila/blob/main/docs/reference/formats/json.md)         | 🟢 No loss        | 🟢 No loss    | 🟢 Stable               |                                |
| [JSON5](https://github.com/stencila/stencila/blob/main/docs/reference/formats/json5.md)       | 🟢 No loss        | 🟢 No loss    | 🟢 Stable               |                                |
| [YAML](https://github.com/stencila/stencila/blob/main/docs/reference/formats/yaml.md)         | 🟢 No loss        | 🟢 No loss    | 🟢 Stable               |                                |
| [Debug](https://github.com/stencila/stencila/blob/main/docs/reference/formats/debug.md)       | 🔷 Low loss       |              | 🟢 Stable               |                                |

## Bindings

The `Include` type is represented in these bindings:

- [JSON-LD](https://stencila.dev/Include.jsonld)
- [JSON Schema](https://stencila.dev/Include.schema.json)
- Python class [`Include`](https://github.com/stencila/stencila/blob/main/python/python/stencila/types/include.py)
- Rust struct [`Include`](https://github.com/stencila/stencila/blob/main/rust/schema/src/types/include.rs)
- TypeScript class [`Include`](https://github.com/stencila/stencila/blob/main/typescript/src/types/Include.ts)

## Source

This documentation was generated from [`Include.yaml`](https://github.com/stencila/stencila/blob/main/schema/Include.yaml) by [`docs.rs`](https://github.com/stencila/stencila/blob/main/rust/schema-gen/src/docs.rs).