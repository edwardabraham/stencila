use once_cell::sync::Lazy;
use parser_treesitter::{
    apply_tags, captures_as_args_map, child_text,
    eyre::Result,
    graph_triples::{relations, resources, Pairs},
    path_utils,
    utils::{is_quoted, remove_quotes},
    Parser, ParserTrait, TreesitterParser,
};
use std::path::Path;

/// Tree-sitter based parser for R
static PARSER: Lazy<TreesitterParser> =
    Lazy::new(|| TreesitterParser::new(tree_sitter_r::language(), QUERY));

/// Tree-sitter AST query
const QUERY: &str = r#"
(program (comment) @comment)

(call
    function: (identifier) @function (#match? @function "^library|require$")
    arguments:(
        arguments
            ([(identifier)(string)] @arg)*
            (
                (identifier) @arg_name
                [(true)(false)] @arg_value
            )*
    )
)

(call
    function: (identifier) @function (#match? @function "^read\.")
    arguments: [
        (
            arguments
                .
                value: (string) @arg
        )
        (
            arguments
                name: (identifier) @arg_name
                .
                value: (string) @arg_value
        )
    ]
)

(call
    function: (identifier) @function (#match? @function "^write\.")
    arguments: [
        (
            arguments
                .
                value: (_) @arg
                .
                value: (string) @arg
        )
        (
            arguments
                name: (identifier) @arg_name
                .
                value: (string) @arg_value
        )
    ]
)

(program [
    (left_assignment name: (identifier) @identifer value: (_) @value)
    (equals_assignment name: (identifier) @identifer value: (_) @value)
])
(super_assignment name: (identifier) @identifer  value: (_) @value)

((identifier) @identifer)
"#;

mod ignores;
use ignores::USE_IGNORE;

/// A parser for Python
pub struct RParser {}

impl ParserTrait for RParser {
    fn spec() -> Parser {
        Parser {
            language: "r".to_string(),
        }
    }

    fn parse(path: &Path, code: &str) -> Result<Pairs> {
        let code = code.as_bytes();
        let tree = PARSER.parse(code);
        let matches = PARSER.query(code, &tree);

        let relations = matches
            .iter()
            .filter_map(|(pattern, captures)| match pattern {
                1 => {
                    // Imports a package using `library` or `require`
                    let args = captures_as_args_map(captures);
                    args.get("0")
                        .or_else(|| args.get("package"))
                        .and_then(|package| {
                            if let Some(is_char) = args.get("character.only") {
                                if is_char.text.starts_with('T') && !is_quoted(&package.text) {
                                    return None;
                                }
                            } else if is_quoted(&package.text) {
                                return None;
                            }
                            Some((
                                relations::uses(package.range),
                                resources::module("r", &remove_quotes(&package.text)),
                            ))
                        })
                }
                2 => {
                    // Reads a file
                    let args = captures_as_args_map(captures);
                    args.get("0").or_else(|| args.get("file")).map(|file| {
                        (
                            relations::reads(file.range),
                            resources::file(&path_utils::merge(path, remove_quotes(&file.text))),
                        )
                    })
                }
                3 => {
                    // Writes a file
                    let args = captures_as_args_map(captures);
                    args.get("1").or_else(|| args.get("file")).map(|file| {
                        (
                            relations::writes(file.range),
                            resources::file(&path_utils::merge(path, remove_quotes(&file.text))),
                        )
                    })
                }
                4 | 5 => {
                    // Assigns a symbol at the top level of the module
                    let range = captures[0].range;
                    let name = captures[0].text.clone();
                    let value = captures[1].node;
                    let kind = match value.kind() {
                        "true" | "false" => "Boolean",
                        "integer" => "Integer",
                        "float" => "Number",
                        "string" => "String",
                        "function_definition" => "Function",
                        "call" => match child_text(value, "function", code) {
                            "data.frame" | "read.csv" | "read.csv2" | "read.delim"
                            | "read.table" => "Datatable",
                            _ => "",
                        },
                        _ => "",
                    };
                    Some((
                        relations::assigns(range),
                        resources::symbol(path, &name, kind),
                    ))
                }
                6 => {
                    // Uses a function or variable
                    let node = captures[0].node;
                    let range = captures[0].range;
                    let symbol = captures[0].text.clone();

                    let mut parent = node.parent();
                    while let Some(parent_node) = parent {
                        match parent_node.kind() {
                            // Skip identifiers that are the `name` of an assignment
                            "left_assignment" | "equals_assignment" | "super_assignment" => {
                                if let Some(name) = parent_node.child_by_field_name("name") {
                                    if name == node {
                                        return None;
                                    }
                                }
                            }
                            // Skip identifiers that are the `name` of a function call argument
                            "arguments" => {
                                let mut cursor = node.walk();
                                for name in parent_node.children_by_field_name("name", &mut cursor)
                                {
                                    if name == node {
                                        return None;
                                    }
                                }
                            }
                            // Skip identifiers that are an `attribute`
                            "dollar" => {
                                // The second child of the `dollar` should be ignored
                                if Some(node) == parent_node.child(2) {
                                    return None;
                                }
                            }
                            // Skip identifiers that are the `name` of a for loop, or that refer to it
                            "for" => {
                                if let Some(name) = parent_node.child_by_field_name("name") {
                                    if name == node || name.utf8_text(code).unwrap() == symbol {
                                        return None;
                                    }
                                }
                            }
                            // Skip package identifiers
                            "call" => {
                                let name = child_text(parent_node, "function", code);
                                if name == "library" || name == "require" {
                                    return None;
                                }
                            }
                            // Skip identifiers within a function definition
                            "function_definition" => return None,
                            _ => {}
                        }
                        parent = parent_node.parent();
                    }

                    let resource = match node.parent() {
                        Some(parent_node) => match parent_node.kind() {
                            "call" => {
                                // Because there are so many globals, unlike for other languages
                                // we only ignore apparent uses of global functions in calls.
                                // This avoids false negatives when a variable has been given a
                                // global name (e.g. "data").
                                if USE_IGNORE.contains(&symbol.as_str()) {
                                    return None;
                                }
                                resources::symbol(path, &symbol, "Function")
                            }
                            _ => resources::symbol(path, &symbol, ""),
                        },
                        None => resources::symbol(path, &symbol, ""),
                    };

                    Some((relations::uses(range), resource))
                }
                _ => None,
            })
            .collect();

        let pairs = apply_tags(path, &Self::spec().language, matches, 0, relations);
        Ok(pairs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_snaps::{insta::assert_json_snapshot, snapshot_fixtures};
    use test_utils::fixtures;

    #[test]
    fn parse_r_fragments() {
        snapshot_fixtures("fragments/r/*.R", |path| {
            let code = std::fs::read_to_string(path).expect("Unable to read");
            let path = path.strip_prefix(fixtures()).expect("Unable to strip");
            let pairs = RParser::parse(path, &code).expect("Unable to parse");
            assert_json_snapshot!(pairs);
        })
    }
}