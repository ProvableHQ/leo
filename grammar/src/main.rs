// Copyright (C) 2019-2021 Aleo Systems Inc.
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
use abnf::types::{Node, Rule};
use anyhow::Result;
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

        Processor {
            grammar,
            line: 0,
            out: String::new(),
            rules,
            scope: Scope::Free,
        }
    }

    /// Main function for this struct.
    /// Goes through each line and transforms it into proper markdown.
    fn process(&mut self) -> Result<()> {
        let mut lines = self.grammar.lines();
        let mut prev = "";

        while let Some(line) = lines.next() {
            self.line += 1;

            // code block in comment (not highlighted as abnf)
            if line.starts_with(";  ") {
                self.enter_scope(Scope::Code)?;
                self.append_str(&line[3..]);

            // just comment. end of code block
            } else if line.starts_with("; ") {
                self.enter_scope(Scope::Free)?;
                self.append_str(&line[2..]);

            // horizontal rule - section separator
            } else if line.starts_with(";;;;;;;;;;") {
                self.enter_scope(Scope::Free)?;
                self.append_str("\n--------\n");

            // empty line in comment. end of code block
            } else if line.starts_with(";") {
                self.enter_scope(Scope::Free)?;
                self.append_str("\n\n");

            // just empty line. end of doc, start of definition
            } else if line.len() == 0 {
                self.enter_scope(Scope::Free)?;
                self.append_str("");

            // definition (may be multiline)
            } else {
                // if there's an equality sign and previous line was empty
                if line.contains("=") && prev.len() == 0 {
                    let (def, _) = line.split_at(line.find("=").unwrap());
                    let def = def.trim();

                    // try to find rule matching definition or fail
                    let rule = self.rules.get(&def.to_string()).map(|rule| rule.clone()).unwrap();

                    self.enter_scope(Scope::Definition(rule))?;
                }

                self.append_str(line);
            }

            prev = line;
        }

        Ok(())
    }

    /// Append new line into output, add newline character.
    fn append_str(&mut self, line: &str) {
        self.out.push_str(line);
        self.out.push_str("\n");
    }

    /// Enter new scope (definition or code block). Allows customizing
    /// pre and post lines for each scope entered or exited.
    fn enter_scope(&mut self, new_scope: Scope) -> Result<()> {
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
                // 3. join results as a list
                // Note: GitHub only allows custom tags with 'user-content-' prefix
                let keys = rules
                    .into_iter()
                    .collect::<HashSet<_>>()
                    .into_iter()
                    .map(|tag| format!("[{}](#user-content-{})", &tag, tag))
                    .collect::<Vec<String>>()
                    .join(", ");

                self.append_str("```");

                if keys.len() > 0 {
                    self.append_str(&format!("\nGo to: _{}_;\n", keys));
                }
            }
            (_, _) => (),
        };

        self.scope = new_scope;

        Ok(())
    }
}

/// Recursively parse ABNF Node and fill sum vec with found rule names.
fn parse_abnf_node(node: &Node, sum: &mut Vec<String>) {
    match node {
        // these two are just vectors of rules
        Node::Alternation(vec) | Node::Concatenation(vec) => {
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
    let grammar = include_str!("../abnf-grammar.txt");

    // A. Coglio's proposal for %s syntax for case-sensitive statements has not been implemented
    // in this library, so we need to remove all occurrences of %s in the grammar file.
    // Link to this proposal: https://www.kestrel.edu/people/coglio/vstte18.pdf
    let grammar = &str::replace(grammar, "%s", "");

    // Parse ABNF to get list of all definitions.
    let parsed = abnf::rulelist(grammar).map_err(|e| {
        eprintln!("{}", &e);
        anyhow::anyhow!(e)
    })?;

    // Init parser and run it. That's it.
    let mut parser = Processor::new(grammar, parsed);
    parser.process()?;

    // Print result of conversion to STDOUT.
    println!("{}", parser.out);

    Ok(())
}
