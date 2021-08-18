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

//! Compiles a Leo program from a file path.

use crate::{CombineAstAsgDirector, CombinerOptions};
use leo_asg::Program as AsgProgram;
use leo_ast::{Ast, Program as AstProgram, ReconstructingReducer};
use leo_errors::Result;

macro_rules! phase {
    ($phase_name:ident, $function:item) => {
        pub struct $phase_name {
            in_circuit: bool,
        }

        pub struct Options;

        impl CombinerOptions for Options {
            $function
        }

        impl ReconstructingReducer for $phase_name {
            fn in_circuit(&self) -> bool {
                self.in_circuit
            }

            fn swap_in_circuit(&mut self) {
                self.in_circuit = !self.in_circuit;
            }
        }

        impl Default for $phase_name {
            fn default() -> Self {
                Self { in_circuit: false }
            }
        }

        impl $phase_name {
            pub fn phase_ast(&self, ast: &AstProgram, asg: &AsgProgram) -> Result<Ast> {
                Ok(Ast::new(CombineAstAsgDirector::new(Self::default(), Options{})
                    .reduce_program(ast, asg)?))
            }
        }
    };
}

phase!(
    TypeInferencePhase,
    fn type_inference_enabled(&self) -> bool {
        true
    }
);
