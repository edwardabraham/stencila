<!-- Generated from doc comments in Rust. Do not edit. -->

# `tasks`: List the code execution tasks in a document kernel space

## Usage

```sh
stencila documents tasks [options] <path>
```

## Arguments

| Name   | Description                   |
| ------ | ----------------------------- |
| `path` | The path of the document file |

## Options

| Name                   | Description                                                                                                                            |
| ---------------------- | -------------------------------------------------------------------------------------------------------------------------------------- |
| `--format -f <format>` | The format of the document file.                                                                                                       |
| `--num -n <num>`       | The maximum number of tasks to show. Default: 100                                                                                      |
| `--sort -s <sort>`     | The order to sort tasks (defaults to by task number). One of: `number`, `created`, `started`, `finished`, `cancelled`. Default: number |
| `--desc -d`            | Whether to sort in descending order.                                                                                                   |
| `--kernel -k <kernel>` | Only show tasks assigned to a specific kernel.                                                                                         |

## Global options

| Name                        | Description                                                                                                                                          |
| --------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------- |
| `--help`                    | Print help information.                                                                                                                              |
| `--version`                 | Print version information.                                                                                                                           |
| `--as <format>`             | Format to display output values (if possible).                                                                                                       |
| `--json`                    | Display output values as JSON (alias for `--as json`).                                                                                               |
| `--yaml`                    | Display output values as YAML (alias for `--as yaml`).                                                                                               |
| `--md`                      | Display output values as Markdown if possible (alias for `--as md`).                                                                                 |
| `--interact -i`             | Enter interactive mode (with any command and options as the prefix).                                                                                 |
| `--debug`                   | Print debug level log events and additional diagnostics. Equivalent to setting `--log-level=debug` and `--log-format=detail` and overrides the both. |
| `--log-level <log-level>`   | The minimum log level to print. One of: `trace`, `debug`, `info`, `warn`, `error`, `never`                                                           |
| `--log-format <log-format>` | The format to print log events. One of: `simple`, `detail`, `json`                                                                                   |