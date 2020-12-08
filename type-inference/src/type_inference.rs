// Copyright (C) 2019-2020 Aleo Systems Inc.
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

use crate::{Frame, Scope, TypeInferenceError};
use leo_ast::{Circuit, CircuitMember, Function, Program};
use leo_symbol_table::SymbolTable;

/// A type inference check for a Leo program.
///
/// A [`TypeInference`] type stores a stack of frames. A new frame is created for every
/// function. Frames store type assertions that assert an expression is a type.
/// Calling the `check()` method on a [`TypeInference`] checks that all type assertions are satisfied.
pub struct TypeInference {
    table: SymbolTable,
    frames: Vec<Frame>,
}

impl TypeInference {
    ///
    /// Creates and runs a new `TypeInference` check on a given program and symbol table.
    ///
    /// Evaluates all `TypeAssertion` predicates.
    ///
    #[allow(clippy::new_ret_no_self)]
    pub fn new(program: &Program, symbol_table: SymbolTable) -> Result<(), TypeInferenceError> {
        let mut type_inference = Self {
            table: symbol_table,
            frames: Vec::new(),
        };

        type_inference.parse_program(program)?;

        type_inference.check()
    }

    ///
    /// Collects a vector of `TypeAssertion` predicates from a program.
    ///
    fn parse_program(&mut self, program: &Program) -> Result<(), TypeInferenceError> {
        // Parse circuit types in program context.
        self.parse_circuits(program.circuits.iter().map(|(_identifier, circuit)| circuit))?;

        // Parse functions in program context.
        self.parse_functions(program.functions.iter().map(|(_identifier, function)| function))
    }

    ///
    /// Collects a vector of `Frames`s from a vector of circuit functions.
    ///
    fn parse_circuits<'a>(&mut self, circuits: impl Iterator<Item = &'a Circuit>) -> Result<(), TypeInferenceError> {
        for circuit in circuits {
            self.parse_circuit(circuit)?;
        }

        Ok(())
    }

    ///
    /// Collects a vector of `Frames`s from a circuit function.
    ///
    /// Each frame collects a vector of `TypeAssertion` predicates from each function.
    ///
    fn parse_circuit(&mut self, circuit: &Circuit) -> Result<(), TypeInferenceError> {
        let name = &circuit.circuit_name.name;

        // Get circuit type from circuit symbol table.
        let circuit_type = self.table.get_circuit_type(name).unwrap().clone();

        // Create a new function for each circuit member function.
        for circuit_member in &circuit.members {
            // ignore circuit member variables
            if let CircuitMember::CircuitFunction(function) = circuit_member {
                // Collect `TypeAssertion` predicates from the function.
                // Pass down circuit self type and circuit variable types to each function.
                let frame = Frame::new_circuit_function(
                    function.to_owned(),
                    circuit_type.clone(),
                    Scope::default(),
                    self.table.clone(),
                )?;

                self.frames.push(frame)
            }
        }

        Ok(())
    }

    ///
    /// Collects a vector of `TypeAssertion` predicates from a vector of functions.
    ///
    fn parse_functions<'a>(&mut self, functions: impl Iterator<Item = &'a Function>) -> Result<(), TypeInferenceError> {
        for function in functions {
            self.parse_function(function)?;
        }

        Ok(())
    }

    ///
    /// Collects a vector of `TypeAssertion` predicates from a function.
    ///
    fn parse_function(&mut self, function: &Function) -> Result<(), TypeInferenceError> {
        let frame = Frame::new_function(function.to_owned(), None, None, self.table.clone())?;

        self.frames.push(frame);

        Ok(())
    }

    ///
    /// Returns the result of evaluating all `TypeAssertion` predicates.
    ///
    /// Will attempt to substitute a `Type` for all `TypeVariable`s.
    /// Returns a `LeoResolvedAst` if all `TypeAssertion` predicates are true.
    /// Returns ERROR if a `TypeAssertion` predicate is false or a solution does not exist.
    ///
    pub fn check(self) -> Result<(), TypeInferenceError> {
        for frame in self.frames {
            frame.check()?;
        }

        Ok(())
    }
}
