// Copyright (C) 2019-2026 Provable Inc.
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

//! Transforms a captured `Block` into a standalone async `Function` plus a
//! corresponding call expression.
//!
//! This pass analyzes symbol accesses inside the block, determines which
//! variables must become parameters, and synthesizes the necessary `Input`s
//! and call-site arguments. Tuple and tuple-field accesses are normalized so
//! that each accessed element becomes a unique parameter, with full-tuple
//! reconstruction when needed.
//!
//! The original block is then reconstructed with all symbol references
//! replaced by these synthesized parameters. The result is a function
//! that encapsulates the block's logic and a call expression that invokes it.
//!
//! # Example
//! ```leo
//! // Original block
//! let a: i32 = 1;
//! let b: i32 = 2;
//! let c: (i32, i32) = (3, 4);
//! {
//!     let x = a + b;
//!     let y = c.0 + c.1;
//!     ..
//! }
//!
//! // Rewritten as a function + call expression (assume `variant` is `AsyncFunction` here)
//! async function generated_async(a: i32, b: i32, "c.0": i32, "c.1": i32) {
//!     let x = a + b;
//!     let y = "c.0" + "c.1";
//!     ..
//! }
//!
//! // Call
//! generated_async(a, b, c.0, c.1);
//! ```

use crate::{CompilerState, Replacer, SymbolAccessCollector};

use leo_ast::{
    AstReconstructor,
    AstVisitor,
    Block,
    CallExpression,
    Expression,
    Function,
    Identifier,
    Input,
    Location,
    Node,
    Path,
    TupleAccess,
    TupleExpression,
    TupleType,
    Type,
    Variant,
};
use leo_span::{Span, Symbol};

use indexmap::IndexMap;

pub struct BlockToFunctionRewriter<'a> {
    state: &'a mut CompilerState,
    current_program: Symbol,
}

impl<'a> BlockToFunctionRewriter<'a> {
    pub fn new(state: &'a mut CompilerState, current_program: Symbol) -> Self {
        Self { state, current_program }
    }
}

impl BlockToFunctionRewriter<'_> {
    pub fn rewrite_block(
        &mut self,
        input: &Block,
        function_name: Symbol,
        function_variant: Variant,
    ) -> (Function, Expression) {
        // Collect all symbol accesses in the block.
        let mut access_collector = SymbolAccessCollector::new(self.state);
        access_collector.visit_block(input);

        // Stores mapping from accessed symbol (and optional index) to the expression used in replacement.
        let mut replacements: IndexMap<(Symbol, Option<usize>), Expression> = IndexMap::new();

        // Helper to create a fresh `Identifier`.
        let make_identifier = |slf: &mut Self, symbol: Symbol| Identifier {
            name: symbol,
            span: Span::default(),
            id: slf.state.node_builder.next_id(),
        };

        // Generates a set of `Input`s and corresponding call-site `Expression`s for a given symbol access.
        //
        // This function handles both:
        // - Direct variable accesses (e.g., `foo`)
        // - Tuple element accesses (e.g., `foo.0`)
        //
        // For tuple accesses:
        // - If a single element (e.g. `foo.0`) is accessed, it generates a synthetic input like `"foo.0"`.
        // - If the whole tuple (e.g. `foo`) is accessed, it ensures all elements are covered by:
        //     - Reusing existing inputs from `replacements` if already generated via prior field access.
        //     - Creating new inputs and arguments for any missing elements.
        // - The entire tuple is reconstructed in `replacements` using the individual elements as a `TupleExpression`.
        //
        // This function also ensures deduplication by consulting the `replacements` map:
        // - If a given `(symbol, index)` has already been processed, no duplicate input or argument is generated.
        // - This prevents repeated parameters for accesses like both `foo` and `foo.0`.
        //
        // # Parameters
        // - `symbol`: The symbol being accessed.
        // - `var_type`: The type of the symbol (may be a tuple or base type).
        // - `index_opt`: `Some(index)` for a tuple field (e.g., `.0`), or `None` for full-variable access.
        //
        // # Returns
        // A `Vec<(Input, Expression)>`, where:
        // - `Input` is a parameter for the generated function.
        // - `Expression` is the call-site argument expression used to invoke that parameter.
        let mut make_inputs_and_arguments =
            |slf: &mut Self, symbol: Symbol, var_type: &Type, index_opt: Option<usize>| -> Vec<(Input, Expression)> {
                if replacements.contains_key(&(symbol, index_opt)) {
                    return vec![]; // No new input needed; argument already exists
                }

                match index_opt {
                    Some(index) => {
                        let Type::Tuple(TupleType { elements }) = var_type else {
                            panic!("Expected tuple type when accessing tuple field: {symbol}.{index}");
                        };

                        let synthetic_name = format!("\"{symbol}.{index}\"");
                        let synthetic_symbol = Symbol::intern(&synthetic_name);
                        let identifier = make_identifier(slf, synthetic_symbol);

                        let input = Input {
                            identifier,
                            mode: leo_ast::Mode::None,
                            type_: elements[index].clone(),
                            span: Span::default(),
                            id: slf.state.node_builder.next_id(),
                        };

                        replacements.insert((symbol, Some(index)), Path::from(identifier).to_local().into());

                        vec![(
                            input,
                            TupleAccess {
                                tuple: Path::from(make_identifier(slf, symbol)).to_local().into(),
                                index: index.into(),
                                span: Span::default(),
                                id: slf.state.node_builder.next_id(),
                            }
                            .into(),
                        )]
                    }

                    None => match var_type {
                        Type::Tuple(TupleType { elements }) => {
                            let mut inputs_and_arguments = Vec::with_capacity(elements.len());
                            let mut tuple_elements = Vec::with_capacity(elements.len());

                            for (i, element_type) in elements.iter().enumerate() {
                                let key = (symbol, Some(i));

                                // Skip if this field is already handled
                                if let Some(existing_expr) = replacements.get(&key) {
                                    tuple_elements.push(existing_expr.clone());
                                    continue;
                                }

                                // Otherwise, synthesize identifier and input
                                let synthetic_name = format!("\"{symbol}.{i}\"");
                                let synthetic_symbol = Symbol::intern(&synthetic_name);
                                let identifier = make_identifier(slf, synthetic_symbol);

                                let input = Input {
                                    identifier,
                                    mode: leo_ast::Mode::None,
                                    type_: element_type.clone(),
                                    span: Span::default(),
                                    id: slf.state.node_builder.next_id(),
                                };

                                let expr: Expression = Path::from(identifier).to_local().into();

                                replacements.insert(key, expr.clone());
                                tuple_elements.push(expr.clone());
                                inputs_and_arguments.push((
                                    input,
                                    TupleAccess {
                                        tuple: Path::from(make_identifier(slf, symbol)).to_local().into(),
                                        index: i.into(),
                                        span: Span::default(),
                                        id: slf.state.node_builder.next_id(),
                                    }
                                    .into(),
                                ));
                            }

                            // Now insert the full tuple (even if all fields were already there).
                            replacements.insert(
                                (symbol, None),
                                Expression::Tuple(TupleExpression {
                                    elements: tuple_elements,
                                    span: Span::default(),
                                    id: slf.state.node_builder.next_id(),
                                }),
                            );

                            inputs_and_arguments
                        }

                        _ => {
                            let identifier = make_identifier(slf, symbol);
                            let input = Input {
                                identifier,
                                mode: leo_ast::Mode::None,
                                type_: var_type.clone(),
                                span: Span::default(),
                                id: slf.state.node_builder.next_id(),
                            };

                            replacements.insert((symbol, None), Path::from(identifier).to_local().into());

                            let argument = Path::from(make_identifier(slf, symbol)).to_local().into();
                            vec![(input, argument)]
                        }
                    },
                }
            };

        // Resolve symbol accesses into inputs and call arguments.
        let (inputs, arguments): (Vec<_>, Vec<_>) = access_collector
            .symbol_accesses
            .iter()
            .filter_map(|(path, index)| {
                // Skip globals and variables that are local to this block or to one of its children.

                // Skip globals.
                if path.is_global() {
                    return None;
                }

                // Skip variables that are local to this block or to one of its children.
                let local_var_name = path.expect_local_symbol(); // Not global, so must be local 
                if self.state.symbol_table.is_local_to_or_in_child_scope(input.id(), local_var_name) {
                    return None;
                }

                // All other variables become parameters to the function being built.
                let var = self.state.symbol_table.lookup_local(local_var_name)?;
                Some(make_inputs_and_arguments(self, local_var_name, &var.type_.expect("must be known by now"), *index))
            })
            .flatten()
            .unzip();

        // Replacement logic used to patch the block.
        let replace_expr = |expr: &Expression| -> Expression {
            match expr {
                Expression::Path(path) => {
                    replacements.get(&(path.identifier().name, None)).cloned().unwrap_or_else(|| expr.clone())
                }

                Expression::TupleAccess(ta) => {
                    if let Expression::Path(path) = &ta.tuple {
                        replacements
                            .get(&(path.identifier().name, Some(ta.index.value())))
                            .cloned()
                            .unwrap_or_else(|| expr.clone())
                    } else {
                        expr.clone()
                    }
                }

                _ => expr.clone(),
            }
        };

        // Reconstruct the block with replaced references.
        let mut replacer = Replacer::new(replace_expr, true /* refresh IDs */, self.state);
        let new_block = replacer.reconstruct_block(input.clone()).0;

        // Define the new function.
        let function = Function {
            annotations: vec![],
            variant: function_variant,
            identifier: make_identifier(self, function_name),
            const_parameters: vec![],
            input: inputs,
            output: vec![],          // No returns supported yet.
            output_type: Type::Unit, // No returns supported yet.
            block: new_block,
            span: input.span,
            id: self.state.node_builder.next_id(),
        };

        // Create the call expression to invoke the function.
        let call_to_function = CallExpression {
            function: Path::from(make_identifier(self, function_name))
                .to_global(Location::new(self.current_program, vec![function_name])),
            const_arguments: vec![],
            arguments,
            span: input.span,
            id: self.state.node_builder.next_id(),
        };

        (function, call_to_function.into())
    }
}
