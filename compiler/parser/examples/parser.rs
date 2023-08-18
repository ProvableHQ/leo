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

#![forbid(unsafe_code)]

use leo_ast::{Ast, NodeBuilder};
use leo_errors::emitter::Handler;
use leo_span::symbol::create_session_if_not_set_then;

use clap::Parser;
use std::{
    fs,
    path::{Path, PathBuf},
};

#[derive(Debug, Parser)]
#[clap(name = "leo parser", about = "Parse Leo AST and store it as a JSON")]
struct Opt {
    /// Path to the Leo file.
    input_path: PathBuf,

    /// Optional path to the output directory.
    out_dir_path: Option<PathBuf>,

    /// Whether to print result to STDOUT.
    #[clap(short, long)]
    print_stdout: bool,
}

fn main() -> Result<(), String> {
    let opt = Opt::parse();
    // Parses the Leo file constructing an ast which is then serialized.
    let serialized_leo_tree = create_session_if_not_set_then(|s| {
        let code = s.source_map.load_file(&opt.input_path).expect("failed to open file");

        Handler::with(|h| {
            let node_builder = NodeBuilder::default();
            let ast = leo_parser::parse_ast(h, &node_builder, &code.src, code.start_pos)?;
            let json = Ast::to_json_string(&ast)?;
            println!("{json}");
            Ok(json)
        })
        .map_err(|b| b.to_string())
    })?;

    if opt.print_stdout {
        println!("{serialized_leo_tree}");
    }

    let out_path = if let Some(out_dir) = opt.out_dir_path {
        format!("{}/{}.json", out_dir.as_path().display(), opt.input_path.file_stem().unwrap().to_str().unwrap())
    } else {
        format!("./{}.json", opt.input_path.file_stem().unwrap().to_str().unwrap())
    };

    fs::write(Path::new(&out_path), serialized_leo_tree).expect("failed to write output");

    Ok(())
}
