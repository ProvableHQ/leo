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

use leo_ast::NodeBuilder;
use leo_errors::{emitter::Handler, Result};
use leo_span::symbol::create_session_if_not_set_then;

use clap::Parser;
use std::{
    fs,
    path::{Path, PathBuf},
};

#[derive(Debug, Parser)]
#[clap(name = "input parser", about = "Parse an Input file and save its JSON representation")]
struct Opt {
    /// Path to the input file.
    input_path: PathBuf,

    /// Optional path to the output directory.
    out_dir_path: Option<PathBuf>,

    /// Whether to print result to STDOUT.
    #[clap(short, long)]
    print_stdout: bool,
}

fn main() -> Result<(), String> {
    let opt = Opt::parse();
    let input_tree = create_session_if_not_set_then(|s| {
        let input_string = s.source_map.load_file(&opt.input_path).expect("failed to open an input file");

        Handler::with(|handler| {
            let node_builder = NodeBuilder::default();
            let input =
                leo_parser::parse_program_inputs(handler, &node_builder, &input_string.src, input_string.start_pos)?;
            input.to_json_string()
        })
        .map_err(|e| e.to_string())
    })?;

    if opt.print_stdout {
        println!("{input_tree}");
    }

    let out_path = if let Some(out_dir) = opt.out_dir_path {
        format!("{}/{}.json", out_dir.as_path().display(), opt.input_path.file_stem().unwrap().to_str().unwrap())
    } else {
        format!("./{}.json", opt.input_path.file_stem().unwrap().to_str().unwrap())
    };

    fs::write(Path::new(&out_path), input_tree).expect("failed to write output");

    Ok(())
}
