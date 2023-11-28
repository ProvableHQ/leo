// Copyright (C) 2019-2023 Aleo Systems Inc.
// This file is part of the Leo library.

// The Leo library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Leo library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Leo library. If not, see <https://www.gnu.org/licenses/>.

// ABNF PARSING RULES
//
// Header:
// ```abnf
// ; Introduction
// ; -------------
// ```
//
// Code block in docs (note double whitespace after colon):
// ```abnf
// ;  code
// ;  code
//```
//
// Rule:
// ```abnf
// address = "address"
// ```
//
// Line:
// ``` abnf
// ;;;;;;;;;
// ```
//

#![forbid(unsafe_code)]

use abnf::types::{Node, Rule};
use anyhow::{anyhow, Result};
use std::collections::{HashMap, HashSet};

/// Processor's scope. Used when code block or definition starts or ends.
#[derive(Debug, Clone)]
enum Scope {
    Free,
    Code,
    Definition(Rule),
}

/// Transforms abnf file into Markdown.
#[derive(Debug, Clone)]
struct Processor<'a> {
    rules: HashMap<String, Rule>,
    grammar: &'a str,
    scope: Scope,
    line: u32,
    out: String,
}

impl<'a> Processor<'a> {
    fn new(grammar: &'a str, abnf: Vec<Rule>) -> Processor<'a> {
        // we need a hashmap to pull rules easily
        let rules: HashMap<String, Rule> = abnf.into_iter().map(|rule| (rule.name().to_string(), rule)).collect();

        Processor { grammar, line: 0, out: String::new(), rules, scope: Scope::Free }
    }

    /// Main function for this struct.
    /// Goes through each line and transforms it into proper markdown.
    fn process(&mut self) {
        let lines = self.grammar.lines();
        let mut prev = "";

        for line in lines {
            self.line += 1;

            // code block in comment (not highlighted as abnf)
            if let Some(code) = line.strip_prefix(";  ") {
                self.enter_scope(Scope::Code);
                self.append_str(code);

            // just comment. end of code block
            } else if let Some(code) = line.strip_prefix("; ") {
                self.enter_scope(Scope::Free);
                self.append_str(code);

            // horizontal rule - section separator
            } else if line.starts_with(";;;;;;;;;;") {
                self.enter_scope(Scope::Free);
                self.append_str("\n--------\n");

            // empty line in comment. end of code block
            } else if line.starts_with(';') {
                self.enter_scope(Scope::Free);
                self.append_str("\n\n");

            // just empty line. end of doc, start of definition
            } else if line.is_empty() {
                self.enter_scope(Scope::Free);
                self.append_str("");

            // definition (may be multiline)
            } else {
                // if there's an equality sign and previous line was empty
                if line.contains('=') && prev.is_empty() {
                    let (def, _) = line.split_at(line.find('=').unwrap());
                    let def = def.trim();

                    // try to find rule matching definition or fail
                    let rule = self.rules.get(&def.to_string()).cloned().unwrap();

                    self.enter_scope(Scope::Definition(rule));
                }

                self.append_str(line);
            }

            prev = line;
        }
    }

    /// Append new line into output, add newline character.
    fn append_str(&mut self, line: &str) {
        self.out.push_str(line);
        self.out.push('\n');
    }

    /// Enter new scope (definition or code block). Allows customizing
    /// pre and post lines for each scope entered or exited.
    fn enter_scope(&mut self, new_scope: Scope) {
        match (&self.scope, &new_scope) {
            // exchange scopes between Free and Code
            (Scope::Free, Scope::Code) => self.append_str("```"),
            (Scope::Code, Scope::Free) => self.append_str("```"),
            // exchange scopes between Free and Definition
            (Scope::Free, Scope::Definition(rule)) => {
                self.append_str(&format!("<a name=\"{}\"></a>", rule.name()));
                self.append_str("```abnf");
            }
            (Scope::Definition(rule), Scope::Free) => {
                let mut rules: Vec<String> = Vec::new();
                parse_abnf_node(rule.node(), &mut rules);

                // 1. leave only unique keys
                // 2. map each rule into a link
                // 3. sort the links so they don't keep changing order
                // 4. join results as a list
                // Note: GitHub only allows custom tags with 'user-content-' prefix
                let mut keyvec = rules
                    .into_iter()
                    .collect::<HashSet<_>>()
                    .into_iter()
                    .map(|tag| format!("[{}](#user-content-{tag})", &tag))
                    .collect::<Vec<String>>();
                keyvec.sort();
                let keys = keyvec.join(", ");

                self.append_str("```");
                if !keys.is_empty() {
                    self.append_str(&format!("\nGo to: _{keys}_;\n"));
                }
            }
            (_, _) => (),
        };

        self.scope = new_scope;
    }
}

/// Recursively parse ABNF Node and fill sum vec with found rule names.
fn parse_abnf_node(node: &Node, sum: &mut Vec<String>) {
    match node {
        // these two are just vectors of rules
        Node::Alternatives(vec) | Node::Concatenation(vec) => {
            for node in vec {
                parse_abnf_node(node, sum);
            }
        }
        Node::Group(node) | Node::Optional(node) => parse_abnf_node(node.as_ref(), sum),

        // push rulename if it is known
        Node::Rulename(name) => sum.push(name.clone()),

        // do nothing for other nodes
        _ => (),
    }
}

fn main() -> Result<()> {
    // Take Leo ABNF grammar file.
    let args: Vec<String> = std::env::args().collect();
    let abnf_path = if let Some(path) = args.get(1) {
        std::path::Path::new(path)
    } else {
        return Err(anyhow!("Usage Error: expects one argument to abnf file to convert."));
    };
    let grammar = std::fs::read_to_string(abnf_path)?;

    // Parse ABNF to get list of all definitions.
    // Rust ABNF does not provide support for `%s` (case sensitive strings, part of
    // the standard); so we need to remove all occurrences before parsing.
    let parsed = abnf::rulelist(&str::replace(&grammar, "%s", "")).map_err(|e| {
        eprintln!("{}", &e);
        anyhow::anyhow!(e)
    })?;

    // Init parser and run it. That's it.
    let mut parser = Processor::new(&grammar, parsed);
    parser.process();

    // Print result of conversion to STDOUT.
    println!("{}", parser.out);

    Ok(())
}
