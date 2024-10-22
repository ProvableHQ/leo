// Copyright (C) 2019-2024 Aleo Systems Inc.
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

use super::*;
use std::{
    fs,
    path::Path ,
};
use leo_span::symbol::create_session_if_not_set_then;
use leo_errors::emitter::Handler;
use leo_ast::NodeBuilder;
use snarkvm::prelude::TestnetV0;
use std::fmt::Write;

/// Clean outputs folder command
#[derive(Parser, Debug)]
pub struct Format {
    #[clap(name = "Input", help = "Input file name")]
    pub(crate) input: String,
    #[clap(name = "Output", help = "Output file name")]
    pub(crate) output: String,
}

impl Command for Format {
    type Input = ();
    type Output = ();

    fn log_span(&self) -> Span {
        tracing::span!(tracing::Level::INFO, "Leo")
    }

    fn prelude(&self, _: Context) -> Result<Self::Input> {
        Ok(())
    }

    fn apply(self, _: Context, _: Self::Input) -> Result<()> {
        let input_str = self.input.as_str();
        let output_str = self.output.as_str();
        let input_path = Path::new(&input_str);
        let output_path = Path::new(&output_str);
        // Parses the Leo file constructing an ast which is then serialized.
        let serialized_leo_tree = create_session_if_not_set_then(|s| {
            let code = s.source_map.load_file(input_path).expect("failed to open file");
            Handler::with(|h| {
                let node_builder = NodeBuilder::default();
                let cst = leo_parser::cst::parse::<TestnetV0>(h, &node_builder, &code.src, code.start_pos)?;
                let mut output = String::new();
                write!(output, "{}", cst.cst).unwrap();
                Ok(output)
            })
            .map_err(|b| b.to_string())
        });
        match serialized_leo_tree {
            Ok(tree) => {
                fs::write(Path::new(output_path), tree).expect("failed to write output");
            },
            Err(e) => eprintln!("Error: {}", e),
        };
        
        Ok(())
    }
}
