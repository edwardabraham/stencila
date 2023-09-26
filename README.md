<div align="center">
  <img src="docs/images/stencila.png" alt="Stencila" width=300>
</div>
<br>

<p align="center">
	<strong>Programmable, reproducible, interactive documents</strong>
</p>

<p align="center">
  <a href="#-introduction">
    👋 Intro
  </a> •
  <a href="#-roadmap">
    🚴 Roadmap
  </a> •
  <a href="#-documentation">
    📜 Docs
  </a> •
  <a href="#-install">
    📥 Install
  </a> •
  <a href="#%EF%B8%8F-develop">
    🛠️ Develop
  </a>
</p>
<p align="center">
  <a href="#-acknowledgements">
    🙏 Acknowledgements
  </a> •
  <a href="#-supporters">
    💖 Supporters
  </a> •
  <a href="#-contributors">
    🙌 Contributors
  </a>
</p>
<br>

<div align="center">
  <a href="#-contributors">
    <img src="https://img.shields.io/github/all-contributors/stencila/stencila?color=1d3bd1&labelColor=3219a8">
  </a>
  <a href="https://discord.gg/GADr6Jv">
    <img src="https://img.shields.io/discord/709952324356800523.svg?logo=discord&label=discord&logoColor=66ff66&color=1d3bd1&labelColor=3219a8">
  </a>
  <a href="https://github.com/stencila/stencila/releases">
    <img src="https://img.shields.io/github/v/release/stencila/stencila.svg?color=1d3bd1&labelColor=3219a8">
  </a>
  <a href="https://www.npmjs.com/package/@stencila/types">
    <img src="https://img.shields.io/npm/v/%40stencila%2Ftypes.svg?label=npm%20%40stencila%2Ftypes&color=1d3bd1&labelColor=3219a8">
  </a>
</div>
<br>

## 👋 Introduction

Stencila is a platform for creating and publishing, dynamic, data-driven content. Our aim is to lower the barriers for creating truly programmable documents, and to make it easier to create beautiful, interactive, and semantically rich, articles, web pages and applications from them. Our roots are in scientific communication, but our tools are useful far beyond.

This is `v2` of Stencila, a rewrite in Rust focussed on the synergies between three recent and impactful innovations and trends:

- [Conflict-free replicated data types (CRDTs)](https://crdt.tech/), and specifically the production ready, Rust-based [Automerge](https://github.com/automerge/automerge), for de-centralized collaboration and version control.

- [Large language models (LLMs)](https://en.wikipedia.org/wiki/Large_language_model) for assisting in writing and editing, prose and code.

- The blurring of the lines between documents and applications as seen in tools such as [Notion](https://notion.com) and [Coda](https://coda.io/).

We are embarking on a rewrite because CRDTs will now be the foundational synchronization and storage layer for Stencila documents. This requires fundamental changes to most other parts of the platform (e.g. how changes are applied to dynamic documents). Furthermore, a rewrite allow us to bake in, rather than bolt on, new modes of interaction between authors and LLM assistants and add mechanisms to mitigate the risks associated with using LLMs (e.g. by recording the actor, human or LLM, that made the change to a document). Much of the code in the [`v1` branch](https://github.com/stencila/stencila/tree/v1) will be reused (after some tidy-ups and refactoring), so `v2` is not a _complete_ rewrite.

## 🚴 Roadmap

Our general strategy is to iterate horizontally across the feature set, rather than fully developing features sequentially. We're less likely to find ourselves painted into an architectural corner with this approach. So expect initial iterations to have limited functionality and be buggy.

We'll be making alpha and beta releases of `v2` early and often across all products (e.g. CLI, desktop, language bindings). We're aiming for a `2.0.0` release by the end of Q3 2024.

🟢 Stable • 🔶 Beta • ⚠️ Alpha • 🚧 Under development • 🧪 Experimental • 🧭 Planned • ❔ Maybe

### Schema

The Stencila Schema is the data model for Stencila documents. Most of the schema is well defined but some document node types are still marked as under development. A summary by category:

| Category | Description                                                                       | Status                                            |
| -------- | --------------------------------------------------------------------------------- | ------------------------------------------------- |
| Works    | Types of creative works (e.g. `Article`, `Figure`, `Review`)                      | 🟢 Stable; mostly based on schema.org             |
| Prose    | Types used in prose (e.g. `Paragraph`, `List`, `Heading`)                         | 🟢 Stable; mostly based on HTML, JATS, Pandoc etc |
| Code     | Types for executable (e.g. `CodeChunk`) and non-executable code (e.g.`CodeBlock`) | 🔶 Beta; may change                               |
| Math     | Types for math symbols and equations (e.g.`MathBlock`)                            | 🔶 Beta; may change                               |
| Data     | Fundamental data types (e.g.`Number`) and validators (e.g. `NumberValidator`)     | 🔶 Beta; may change                               |
| Style    | Types for styling parts of documents (`Span` and `Division`)                      | 🚧 Under development; likely to change            |
| Flow     | Types for document control flow (e.g. `If`, `For`, `Call`)                        | 🚧 Under development; likely to change            |

### Storage and synchronization

In `v2` documents can be stored as binary Automerge CRDT files, forked and merged, and with the ability to import and export the document in various formats. Collaboration, including real-time, is made possible by exchanging fine-grained changes to the CRDT over the network. In addition, we want to enable interoperability with a Git-based workflow.

| Functionality                | Description                                                                                                                                                   | Status                       |
| ---------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------- |
| Documents read/write-able    | Able to write a Stencila document to an Automerge binary file and read it back in                                                                             | ⚠️ Alpha; needs more testing |
| Documents import/export-able | Able to import or export document as alternative formats, using tree diffing to generate CRDT changes                                                         | ⚠️ Alpha; needs more testing |
| Documents fork/merge-able    | Able to create a fork of a document in another file and then later merge with the original                                                                    | 🧭 Planned Q4 2023           |
| Documents diff-able          | Able to view a diff, in any of the supported format, between versions of a document and between a document and another file                                   | 🧭 Planned Q4 2023           |
| Git merge driver             | CLI can act as a [custom Git merge driver](https://www.julianburr.de/til/custom-git-merge-drivers/)                                                           | 🧭 Planned Q4 2023           |
| Relay server                 | Documents can be synchronized by exchanging changes via a relay server                                                                                        | 🧭 Planned Q4 2023           |
| Rendezvous server            | Documents can be synchronized by exchanging changes peer-to-peer using TCP or UDP [hole punching](<https://en.wikipedia.org/wiki/Hole_punching_(networking)>) | ❔ Maybe                     |

### Formats

Interoperability with existing formats has always been a key feature of Stencila. We will bring over _codecs_ (a.k.a. converters) from the `v1` branch and port other functionality from [`encoda`](https://github.com/stencila/encoda) to Rust.

| Format           | Encoding | Decoding | Notes                                                                                                        |
| ---------------- | -------- | -------- | ------------------------------------------------------------------------------------------------------------ |
| JSON             | 🟢       | 🟢       |                                                                                                              |
| JSON5            | 🟢       | 🟢       |                                                                                                              |
| YAML             | 🟢       | 🟢       |                                                                                                              |
| Plain text       | ⚠️       |          |                                                                                                              |
| HTML             | 🚧       | 🧭       |                                                                                                              |
| JATS             | 🚧       | 🚧       | Planned for completion Q4 2023. Port decoding and tests from [`encoda`](https://github.com/stencila/encoda/) |
| Markdown         | 🚧       | 🧭       | Planned Q3 2023 [`v1`](https://github.com/stencila/stencila/tree/v1/rust/codec-md)                           |
| R Markdown       | 🧭       | 🧭       | Relies on Markdown; [`v1`](https://github.com/stencila/stencila/tree/v1/rust/codec-rmd)                      |
| Jupyter Notebook | 🧭       | 🧭       | Relies on Markdown; [`v1`](https://github.com/stencila/stencila/tree/v1/rust/codec-ipynb)                    |
| Scripts          | 🧭       | 🧭       | Relies on Markdown; [`v1`](https://github.com/stencila/stencila/tree/v1/rust/codec-script)                   |
| Pandoc           | 🧭       | 🧭       | Planned Q4 2023. [`v1`](https://github.com/stencila/stencila/tree/v1/rust/codec-pandoc)                      |
| LaTeX            | 🧭       | 🧭       | Relies on Pandoc; [`v1`](https://github.com/stencila/stencila/tree/v1/rust/codec-latex)                      |
| Org              | 🧭       | 🧭       | Relies on Pandoc; [PR](https://github.com/stencila/stencila/pull/1485)                                       |
| Microsoft Word   | 🧭       | 🧭       | Relies on Pandoc; [`v1`](https://github.com/stencila/stencila/tree/v1/rust/codec-docx)                       |
| ODT              | 🧭       | 🧭       | Relies on Pandoc                                                                                             |
| Google Docs      | 🧭       | 🧭       | Planned Q1 2024 [`v1`](https://github.com/stencila/stencila/tree/v1/rust/codec-gdoc)                         |
| PDF              | 🧭       | 🧭       | Planned Q1 2024, relies on HTML; [`v1`](https://github.com/stencila/stencila/tree/v1/rust/codec-pdf)         |

### Kernels

Kernels are what executes the code in Stencila `CodeChunk`s and `CodeExpression`s, as well as in control flow document nodes such as `IfClause` and `For`. In addition to supporting interoperability with existing Jupyter kernels, we will bring over _microkernels_ from `v1`. Microkernels are lightweight kernels for executing code which do not require the user to install anything and which allow for parallelization of execution. We'll also implement at least one kernel for an embedded scripting language so that it is possible to author a Stencila document which does not rely on any other external binary.

| Kernel                | Purpose                                          | Status                                                                                                                                                                                           |
| --------------------- | ------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Embedded lang kernel  | Default language for executable code             | 🧭 Planned Q4 2023. Probably [Rune or Rhai](https://www.boringcactus.com/2020/09/16/survey-of-rust-embeddable-scripting-languages.html) but could be [RustPython](https://rustpython.github.io/) |
| Bash microkernel      | Execute Bash code in documents                   | 🧭 Planned Q4 2023; [`v1`](https://github.com/stencila/stencila/tree/v1/rust/kernel-bash)                                                                                                        |
| Zsh microkernel       | Execute Zsh code in documents                    | 🧭 Planned Q4 2023; [`v1`](https://github.com/stencila/stencila/tree/v1/rust/kernel-zsh)                                                                                                         |
| Python microkernel    | Execute Python code in documents                 | 🧭 Planned Q4 2023; [`v1`](https://github.com/stencila/stencila/tree/v1/rust/kernel-python)                                                                                                      |
| R microkernel         | Execute R code in documents                      | 🧭 Planned Q4 2023; [`v1`](https://github.com/stencila/stencila/tree/v1/rust/kernel-r)                                                                                                           |
| Node.js microkernel   | Execute JavaScript code in documents             | 🧭 Planned Q4 2023; [`v1`](https://github.com/stencila/stencila/tree/v1/rust/kernel-node)                                                                                                        |
| Deno microkernel      | Execute TypeScript code in documents             | ❔ Maybe; [`v1`](https://github.com/stencila/stencila/tree/v1/rust/kernel-deno)                                                                                                                  |
| SQL microkernel       | Execute SQL code in documents                    | 🧭 Planned Q1 2024; [`v1`](https://github.com/stencila/stencila/tree/v1/rust/kernel-sql)                                                                                                         |
| Jupyter kernel bridge | Execute code in Jupyter kernels                  | 🧭 Planned Q1 2024; [`v1`](https://github.com/stencila/stencila/tree/v1/rust/kernel-jupyter)                                                                                                     |
| HTTP kernel           | Interact with RESTful APIs from within documents | ❔ Maybe; [`v1`](https://github.com/stencila/stencila/tree/v1/rust/kernel-http)                                                                                                                  |

### Actors

In Stencila `v2`, non-human changes to the document will be performed, concurrently, by various _actors_. Actors will listen for changes to document and react accordingly. For example, a LLM actor might listen for the insertion of a paragraph starting with "!add a code chunk to read in and summarize mydata.csv" and do just that. We'll be starting by implementing relatively simply actors but to avoid being painted into a corner will probably implement one LLM-base actor relatively early on.

| Actor        | Purpose                                                                                                                                                                                                                              | Status                                                                                         |
| ------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ---------------------------------------------------------------------------------------------- |
| `MathMLer`   | Update the `mathml` property of [`Math`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/math/math.md) nodes when the `code` property changes                                                                   | 🧭 Planned Q4 2023                                                                             |
| `Tailwinder` | Update the `classes` property of [`Styled`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/style/styled.md) nodes when the `code` property changes                                                             | 🧭 Planned Q4 2023 [`v1`](https://github.com/stencila/stencila/tree/v1/rust/parser-tailwind)   |
| Parsers      | Update the `executionDependency` etc properties of [`CodeExecutable`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/code/code-executable.md) nodes when the `code` or `programmingLanguage` properties change | 🧭 Planned Q4 2023 [`v1`](https://github.com/stencila/stencila/tree/v1/rust/parser-treesitter) |
| `Reactor`    | Maintain a dependency graph between nodes and update `executionRequired` of executable nodes when `executionDependency` or `executionStatus` of other nodes changes.                                                                 | 🧭 Planned Q4 2023 [`v1`](https://github.com/stencila/stencila/tree/v1/rust/graph)             |
| `Executor`   | Execute nodes when their `executionRequired` property and update their `executionStatus`, `output`, etc properties                                                                                                                   | 🧭 Planned Q4 2023                                                                             |
| `Coder`      | A LLM actor that creates and edits [`CodeExecutable`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/code/code-executable.md) nodes                                                                            | 🧭 Planned Q1 2024                                                                             |
| `Writer`     | A LLM actor that creates and edits [prose](https://github.com/stencila/stencila/blob/main/docs/reference/schema/prose) nodes                                                                                                         | 🧭 Planned Q1 2024                                                                             |

### Editors

Editors allow users to edit Stencila documents, either directly, or via an intermediate format.

| Interface      | Purpose                                                                                                 | Status                                                                                                          |
| -------------- | ------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------- |
| File watcher   | Edit documents via other formats and tools (e.g. code editors, Microsoft Word) and react on file change | ⚠️ Alpha                                                                                                        |
| Code editor    | Edit documents via other formats using a built-in code editor and react on key presses                  | 🧭 Planned Q4 2023 [`v1`](https://github.com/stencila/stencila/tree/v1/web/src/components/editors/code-editor)  |
| WYSIWYG editor | Edit documents using a built-in WYSIWYG editor and react on key presses and widget interactions         | 🧭 Planned Q1 2024 [`v1`](https://github.com/stencila/stencila/tree/v1/web/src/components/editors/prose-editor) |

### Tools

Tools are what we call the self-contained Stencila products you can download and use locally on your machine to interact with Stencila documents.

| Interface        | Purpose                                                                           | Status                                                       |
| ---------------- | --------------------------------------------------------------------------------- | ------------------------------------------------------------ |
| CLI              | Manage documents from the command line and read and edit them using a web browser | ⚠️ Alpha                                                     |
| Desktop          | Manage, read and edit documents from a desktop app                                | 🧭 Planned Q1 2024, likely using [Tauri](https://tauri.app/) |
| VSCode extension | Manage, read and edit documents from within VSCode                                | ❔ Maybe                                                     |

### Bindings

Bindings allow you to create and manage Stencila document using other programming languages. At this stage we are planning to support Python, Node.js and R but more languages may be added if there is demand.

| Interface  | Description                                                     | Status                                                                                                                                                                    |
| ---------- | --------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Python     | Types and bindings for using Stencila from within Python        | 🚧 In progress, planned for Q3 2023                                                                                                                                       |
| TypeScript | JavaScript classes and TypeScript types for the Stencila Schema | [![](https://img.shields.io/npm/v/%40stencila%2Ftypes.svg?label=npm%20%40stencila%2Ftypes&color=1d3bd1&labelColor=3219a8)](https://www.npmjs.com/package/@stencila/types) |
| Node.js    | Types and bindings for using Stencila from within Node.js       | 🚧 In progress, planned for Q3 2023                                                                                                                                       |
| R          | Types and bindings for using Stencila from within R             | 🧭 Planned Q4 2023                                                                                                                                                        |

## 📜 Documentation

At this stage, documentation for `v2` is mainly reference material, much of it generated:

- [Schema](https://github.com/stencila/stencila/tree/main/docs/reference/schema)
- [Formats](https://github.com/stencila/stencila/tree/main/docs/reference/formats)
- [CLI](https://github.com/stencila/stencila/tree/main/docs/reference/cli.md)

More reference docs as well as guides and tutorial will be added over the coming months. We will be bootstrapping the publishing of all docs (i.e. to use Stencila itself to publish HTML pages) and expect to have an initial published set in Q4 2023.

## 📥 Install

Although `v2` is in early stages of development, and functionality may be limited or buggy, we are releasing binary builds of alpha versions of the Stencila CLI tool and language packages. Doing so allows us to get early feedback and monitor what impact the addition of features has on build times and distribution sizes.

### CLI tool

#### Windows

To install the latest release download `stencila-<version>-x86_64-pc-windows-msvc.zip` from the [latest release](https://github.com/stencila/stencila/releases/latest) and place it somewhere on your `PATH`.

#### MacOS

To install the latest release in `/usr/local/bin`,

```console
curl -L https://raw.githubusercontent.com/stencila/stencila/main/install.sh | bash
```

To install a specific version, append `-s vX.X.X`. Or, if you'd prefer to do it manually, download `stencila-<version>-x86_64-apple-darwin.tar.xz` from the one of the [releases](https://github.com/stencila/stencila/releases) and then,

```console
tar xvf stencila-*.tar.xz
cd stencila-*/
sudo mv -f stencila /usr/local/bin # or wherever you prefer
```

#### Linux

To install the latest release in `~/.local/bin/`,

```console
curl -L https://raw.githubusercontent.com/stencila/stencila/main/install.sh | bash
```

To install a specific version, append `-s vX.X.X`. Or, if you'd prefer to do it manually, download `stencila-<version>-x86_64-unknown-linux-gnu.tar.xz` from the one of the [releases](https://github.com/stencila/stencila/releases) and then,

```console
tar xvf stencila-*.tar.xz
mv -f stencila ~/.local/bin/ # or wherever you prefer
```

#### Docker

The CLI is also available in a Docker image you can pull from the Github Container Registry,

```console
docker pull stencila/stencila
```

and use locally like this for example,

```console
docker run -it --rm -v "$PWD":/work -w /work --network host stencila/stencila --help
```

The same image is also published to the Github Container Registry if you'd prefer to use that,

```console
docker pull ghcr.io/stencila/stencila
```

## 🛠️ Develop

This repository is organized into the following modules. Please see their respective READMEs, where available, for guides to contributing.

- [`schema`](schema): YAML files which define the Stencila Schema, an implementation of, and extensions to, [schema.org](https://schema.org), for programmable documents.

- [`json`](json): A [JSON Schema](https://json-schema.org/) and [JSON LD](https://json-ld.org/) `@context`, generated from Stencila Schema, which can be used to validate Stencila documents and transform them to other vocabularies

- [`rust`](rust): Several Rust crates implementing core functionality and a CLI for working with Stencila documents.

- [`python`](python): A Python package, with classes generated from Stencila Schema and bindings to Rust functions, so you can work with Stencila documents from within Python.

- [`typescript`](typescript): A package of TypeScript types generated from Stencila Schema so you can create type-safe Stencila documents in the browser, Node.js, Deno etc.

- `node`: A Node.js package, using the generated TypeScript types and with runtime validation and bindings to Rust functions, so you can work with Stencila documents from within Node.js.

- [`docs`](docs): Documentation, including reference documentation generated from `schema` and the `rust` CLI tool.

- [`examples`](examples): Example of documents conforming to Stencila Schema, mostly for testing purposes.

## 🙏 Acknowledgements

Stencila is built on the shoulders of many open source projects. Our sincere thanks to all the maintainers and contributors of those projects for their vision, enthusiasm and dedication. But most of all for all their hard work! The following open source projects in particular have an important role in the current version of Stencila. We sponsor these projects where, and to an extent, possible through [GitHub Sponsors](https://github.com/stencila) and [Open Collective](https://opencollective.com/stencila).

|                                                  | Link                                  | Summary                                                                                                                                 |
| ------------------------------------------------ | ------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------- |
| <img src="docs/images/automerge.png" width="80"> | [Automerge](https://automerge.org/)   | A Rust library of data structures for building collaborative applications.                                                              |
| <img src="docs/images/clap.png" width="80">      | [Clap](https://crates.io/crates/clap) | A Command Line Argument Parser for Rust.                                                                                                |
| <img src="docs/images/rust.png" width="80">      | [Rust](https://www.rust-lang.org/)    | A multi-paradigm, high-level, general-purpose programming language which emphasizes performance, type safety, and concurrency.          |
| <img src="docs/images/ferris.png" width="80">    | [Serde](https://serde.rs/)            | A framework for **ser**ializing and **de**serializing Rust data structures efficiently and generically.                                 |
| <img src="docs/images/similar.png" width="80">   | [Similar](https://insta.rs/similar/)  | A Rust library of diffing algorithms including Patience and Hunt–McIlroy / Hunt–Szymanski LCS.                                          |
| <img src="docs/images/tokio.png" width="80">     | [Tokio](https://tokio.rs/)            | An asynchronous runtime for Rust which provides the building blocks needed for writing network applications without compromising speed. |

## 💖 Supporters

We wouldn’t be doing this without the support of these forward looking organizations.

<p align="center"><a href="https://sloan.org/"><img src="docs/images/sloan.png" height="70"></img></a><p>
<p align="center"><a href="https://elifesciences.org/"><img src="docs/images/elife.svg" height="70"></img></a><p>
<p align="center"><a href="https://www.mbie.govt.nz"><img src="docs/images/mbie.jpeg" height="70"></img></a><p>
<p align="center"><a href="https://coko.foundation/"><img src="docs/images/coko.png" height="70"></img></a><p>
<p align="center"><a href="https://www.codeforsociety.org/"><img src="docs/images/css.png" height="70"></img></a><p>
<p align="center"><a href="https://www.callaghaninnovation.govt.nz/"><img src="docs/images/callaghan.png" height="70"></img></a><p>

## 🙌 Contributors

<!-- ALL-CONTRIBUTORS-LIST:START - Do not remove or modify this section -->
<!-- prettier-ignore-start -->
<!-- markdownlint-disable -->
<table>
  <tbody>
    <tr>
      <td align="center" valign="top" width="14.28%"><a href="https://ackerleytng.github.io/blog/"><img src="https://avatars.githubusercontent.com/u/2399902?v=4" width="100px;" alt="Ackerley Tng" title="Ackerley Tng"/></a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/apawlik"><img src="https://avatars2.githubusercontent.com/u/2358535?v=4" width="100px;" alt="Aleksandra Pawlik" title="Aleksandra Pawlik"/></a></td>
      <td align="center" valign="top" width="14.28%"><a href="http://ketch.me"><img src="https://avatars2.githubusercontent.com/u/1646307?v=4" width="100px;" alt="Alex Ketch" title="Alex Ketch"/></a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/beneboy"><img src="https://avatars1.githubusercontent.com/u/292725?v=4" width="100px;" alt="Ben Shaw" title="Ben Shaw"/></a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/colettedoughty"><img src="https://avatars.githubusercontent.com/u/64813273?v=4" width="100px;" alt="Colette Doughty" title="Colette Doughty"/></a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/Integral"><img src="https://avatars.githubusercontent.com/u/182010?v=4" width="100px;" alt="Daniel Beilinson" title="Daniel Beilinson"/></a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://uk.linkedin.com/in/daniel-ecer"><img src="https://avatars.githubusercontent.com/u/1016473?v=4" width="100px;" alt="Daniel Ecer" title="Daniel Ecer"/></a></td>
    </tr>
    <tr>
      <td align="center" valign="top" width="14.28%"><a href="https://orcid.org/0000-0001-9488-1870"><img src="https://avatars.githubusercontent.com/u/465923?v=4" width="100px;" alt="Daniel Mietchen" title="Daniel Mietchen"/></a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://nordholmen.net/"><img src="https://avatars.githubusercontent.com/u/1325054?v=4" width="100px;" alt="Daniel Nüst" title="Daniel Nüst"/></a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/daniellecrobinson"><img src="https://avatars.githubusercontent.com/u/13207169?v=4" width="100px;" alt="Danielle Robinson" title="Danielle Robinson"/></a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/discodavey"><img src="https://avatars.githubusercontent.com/u/745794?v=4" width="100px;" alt="Dave" title="Dave"/></a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/davidcmoulton"><img src="https://avatars.githubusercontent.com/u/2893480?v=4" width="100px;" alt="David Moulton" title="David Moulton"/></a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/fkmendes"><img src="https://avatars.githubusercontent.com/u/10715960?v=4" width="100px;" alt="Fábio H. K. Mendes" title="Fábio H. K. Mendes"/></a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/kayakr"><img src="https://avatars.githubusercontent.com/u/208184?v=4" width="100px;" alt="J Hunt" title="J Hunt"/></a></td>
    </tr>
    <tr>
      <td align="center" valign="top" width="14.28%"><a href="http://toki.io"><img src="https://avatars1.githubusercontent.com/u/10161095?v=4" width="100px;" alt="Jacqueline" title="Jacqueline"/></a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/jamestwebber"><img src="https://avatars.githubusercontent.com/u/963647?v=4" width="100px;" alt="James Webber" title="James Webber"/></a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://juretriglav.si/"><img src="https://avatars.githubusercontent.com/u/238667?v=4" width="100px;" alt="Jure Triglav" title="Jure Triglav"/></a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://larsgw.github.io/"><img src="https://avatars.githubusercontent.com/u/14018963?v=4" width="100px;" alt="Lars Willighagen" title="Lars Willighagen"/></a></td>
      <td align="center" valign="top" width="14.28%"><a href="http://has100ideas.com"><img src="https://avatars0.githubusercontent.com/u/57006?v=4" width="100px;" alt="Mac Cowell" title="Mac Cowell"/></a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/elfring"><img src="https://avatars.githubusercontent.com/u/660477?v=4" width="100px;" alt="Markus Elfring" title="Markus Elfring"/></a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/michael"><img src="https://avatars.githubusercontent.com/u/2931?v=4" width="100px;" alt="Michael Aufreiter" title="Michael Aufreiter"/></a></td>
    </tr>
    <tr>
      <td align="center" valign="top" width="14.28%"><a href="https://moranegg.github.io/"><img src="https://avatars.githubusercontent.com/u/6273005?v=4" width="100px;" alt="Morane Gruenpeter" title="Morane Gruenpeter"/></a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/MorphicResonance"><img src="https://avatars.githubusercontent.com/u/55984436?v=4" width="100px;" alt="MorphicResonance" title="MorphicResonance"/></a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/Cumol"><img src="https://avatars.githubusercontent.com/u/4943380?v=4" width="100px;" alt="Muad Abd El Hay" title="Muad Abd El Hay"/></a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/nokome"><img src="https://avatars0.githubusercontent.com/u/1152336?v=4" width="100px;" alt="Nokome Bentley" title="Nokome Bentley"/></a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/obuchtala"><img src="https://avatars.githubusercontent.com/u/284099?v=4" width="100px;" alt="Oliver Buchtala" title="Oliver Buchtala"/></a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/rgaiacs"><img src="https://avatars.githubusercontent.com/u/1506457?v=4" width="100px;" alt="Raniere Silva" title="Raniere Silva"/></a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://remi.rampin.org/"><img src="https://avatars.githubusercontent.com/u/426784?v=4" width="100px;" alt="Remi Rampin" title="Remi Rampin"/></a></td>
    </tr>
    <tr>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/richlysakowski"><img src="https://avatars.githubusercontent.com/u/9345711?v=4" width="100px;" alt="Rich Lysakowski" title="Rich Lysakowski"/></a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/rgieseke"><img src="https://avatars.githubusercontent.com/u/198537?v=4" width="100px;" alt="Robert Gieseke" title="Robert Gieseke"/></a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://sethvincent.com/"><img src="https://avatars.githubusercontent.com/u/164214?v=4" width="100px;" alt="Seth Vincent" title="Seth Vincent"/></a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/stefanfritsch"><img src="https://avatars.githubusercontent.com/u/7372706?v=4" width="100px;" alt="Stefan Fritsch" title="Stefan Fritsch"/></a></td>
      <td align="center" valign="top" width="14.28%"><a href="http://sirinath.com/"><img src="https://avatars.githubusercontent.com/u/637415?v=4" width="100px;" alt="Suminda Sirinath Salpitikorala Dharmasena" title="Suminda Sirinath Salpitikorala Dharmasena"/></a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://tim.mcnamara.nz/"><img src="https://avatars.githubusercontent.com/u/27889?v=4" width="100px;" alt="Tim McNamara" title="Tim McNamara"/></a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/wooorm"><img src="https://avatars.githubusercontent.com/u/944406?v=4" width="100px;" alt="Titus" title="Titus"/></a></td>
    </tr>
    <tr>
      <td align="center" valign="top" width="14.28%"><a href="http://blog.ouseful.info/"><img src="https://avatars.githubusercontent.com/u/82988?v=4" width="100px;" alt="Tony Hirst" title="Tony Hirst"/></a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/ouboub"><img src="https://avatars.githubusercontent.com/u/5957203?v=4" width="100px;" alt="Uwe Brauer" title="Uwe Brauer"/></a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://vsoch.github.io"><img src="https://avatars0.githubusercontent.com/u/814322?v=4" width="100px;" alt="Vanessasaurus" title="Vanessasaurus"/></a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://neurathsboat.blog/"><img src="https://avatars.githubusercontent.com/u/1205023?v=4" width="100px;" alt="Vassilis Kehayas" title="Vassilis Kehayas"/></a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/alexandr-sisiuc"><img src="https://avatars.githubusercontent.com/u/80925015?v=4" width="100px;" alt="alexandr-sisiuc" title="alexandr-sisiuc"/></a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/asisiuc"><img src="https://avatars0.githubusercontent.com/u/17000527?v=4" width="100px;" alt="asisiuc" title="asisiuc"/></a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/campbellyamane"><img src="https://avatars.githubusercontent.com/u/26848216?v=4" width="100px;" alt="campbellyamane" title="campbellyamane"/></a></td>
    </tr>
    <tr>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/ern0"><img src="https://avatars.githubusercontent.com/u/1758588?v=4" width="100px;" alt="ern0 - Zalka Ernő" title="ern0 - Zalka Ernő"/></a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/grayflow"><img src="https://avatars.githubusercontent.com/u/70041?v=4" width="100px;" alt="grayflow" title="grayflow"/></a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/happydentist"><img src="https://avatars.githubusercontent.com/u/35398116?v=4" width="100px;" alt="happydentist" title="happydentist"/></a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/huang12zheng"><img src="https://avatars.githubusercontent.com/u/28038074?v=4" width="100px;" alt="huang12zheng" title="huang12zheng"/></a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/ignatiusm"><img src="https://avatars.githubusercontent.com/u/11403879?v=4" width="100px;" alt="ignatiusm" title="ignatiusm"/></a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/jm-observer"><img src="https://avatars.githubusercontent.com/u/50091488?v=4" width="100px;" alt="jmhuang" title="jmhuang"/></a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://jonrichter.de/"><img src="https://avatars.githubusercontent.com/u/1645308?v=4" width="100px;" alt="jon r" title="jon r"/></a></td>
    </tr>
    <tr>
      <td align="center" valign="top" width="14.28%"><a href="https://nyanpasu64.gitlab.io/"><img src="https://avatars.githubusercontent.com/u/913957?v=4" width="100px;" alt="kitten" title="kitten"/></a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/solsson"><img src="https://avatars.githubusercontent.com/u/144945?v=4" width="100px;" alt="solsson" title="solsson"/></a></td>
      <td align="center" valign="top" width="14.28%"><a href="http://www.danbeekim.org/"><img src="https://avatars.githubusercontent.com/u/27739141?v=4" width="100px;" alt="taunsquared" title="taunsquared"/></a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/yasirs"><img src="https://avatars.githubusercontent.com/u/400538?v=4" width="100px;" alt="yasirs" title="yasirs"/></a></td>
    </tr>
  </tbody>
</table>

<!-- markdownlint-restore -->
<!-- prettier-ignore-end -->

<!-- ALL-CONTRIBUTORS-LIST:END -->
