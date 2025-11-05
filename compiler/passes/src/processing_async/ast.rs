// Copyright (C) 2019-2025 Provable Inc.
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

use super::ProcessingAsyncVisitor;
use crate::{CompilerState, Replacer};
use indexmap::{IndexMap, IndexSet};
use leo_ast::{
    AstReconstructor,
    AstVisitor,
    AsyncExpression,
    Block,
    CallExpression,
    Expression,
    Function,
    Identifier,
    Input,
    IterationStatement,
    Location,
    Node,
    Path,
    ProgramVisitor,
    Statement,
    TupleAccess,
    TupleExpression,
    TupleType,
    Type,
    Variant,
};
use leo_span::{Span, Symbol};

/// Collects all symbol accesses within an async block,
/// including both direct variable identifiers (`x`) and tuple field accesses (`x.0`, `x.1`, etc.).
/// Each access is recorded as a pair: (Symbol, Option<usize>).
/// - `None` means a direct variable access.
/// - `Some(index)` means a tuple field access.
struct SymbolAccessCollector<'a> {
    state: &'a CompilerState,
    symbol_accesses: IndexSet<(Vec<Symbol>, Option<usize>)>,
}

impl AstVisitor for SymbolAccessCollector<'_> {
    type AdditionalInput = ();
    type Output = ();

    fn visit_path(&mut self, input: &Path, _: &Self::AdditionalInput) -> Self::Output {
        self.symbol_accesses.insert((input.absolute_path(), None));
    }

    fn visit_tuple_access(&mut self, input: &TupleAccess, _: &Self::AdditionalInput) -> Self::Output {
        // Here we assume that we can't have nested tuples which is currently guaranteed by type
        // checking. This may change in the future.
        if let Expression::Path(path) = &input.tuple {
            // Futures aren't accessed by field; treat the whole thing as a direct variable
            if let Some(Type::Future(_)) = self.state.type_table.get(&input.tuple.id()) {
                self.symbol_accesses.insert((path.absolute_path(), None));
            } else {
                self.symbol_accesses.insert((path.absolute_path(), Some(input.index.value())));
            }
        } else {
            self.visit_expression(&input.tuple, &());
        }
    }
}

impl ProgramVisitor for SymbolAccessCollector<'_> {}

impl AstReconstructor for ProcessingAsyncVisitor<'_> {
    type AdditionalInput = ();
    type AdditionalOutput = ();

    /// Transforms an `AsyncExpression` into a standalone async `Function` and returns
    /// a call to this function. This process:
    /// - Collects all referenced symbol accesses in the async block.
    /// - Filters out mappings and constructs typed input parameters.
    /// - Reconstructs an async function with those inputs and the original block.
    /// - Builds and returns a `CallExpression` that invokes the new function.
    fn reconstruct_async(&mut self, input: AsyncExpression, _additional: &()) -> (Expression, Self::AdditionalOutput) {
        // Step 1: Generate a unique name for the async function
        let finalize_fn_name = self.state.assigner.unique_symbol(self.current_function, "$");

        // Step 2: Collect all symbol accesses in the async block
        let mut access_collector = SymbolAccessCollector { state: self.state, symbol_accesses: IndexSet::new() };
        access_collector.visit_async(&input, &());

        // Stores mapping from accessed symbol (and optional index) to the expression used in replacement
        let mut replacements: IndexMap<(Symbol, Option<usize>), Expression> = IndexMap::new();

        // Helper to create a fresh `Identifier`
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
        // - `Input` is a parameter for the generated async function.
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

                        replacements.insert((symbol, Some(index)), Path::from(identifier).into_absolute().into());

                        vec![(
                            input,
                            TupleAccess {
                                tuple: Path::from(make_identifier(slf, symbol)).into_absolute().into(),
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

                                let expr: Expression = Path::from(identifier).into_absolute().into();

                                replacements.insert(key, expr.clone());
                                tuple_elements.push(expr.clone());
                                inputs_and_arguments.push((
                                    input,
                                    TupleAccess {
                                        tuple: Path::from(make_identifier(slf, symbol)).into_absolute().into(),
                                        index: i.into(),
                                        span: Span::default(),
                                        id: slf.state.node_builder.next_id(),
                                    }
                                    .into(),
                                ));
                            }

                            // Now insert the full tuple (even if all fields were already there)
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

                            replacements.insert((symbol, None), Path::from(identifier).into_absolute().into());

                            let argument = Path::from(make_identifier(slf, symbol)).into_absolute().into();
                            vec![(input, argument)]
                        }
                    },
                }
            };

        // Step 3: Resolve symbol accesses into inputs and call arguments
        let (inputs, arguments): (Vec<_>, Vec<_>) = access_collector
            .symbol_accesses
            .iter()
            .filter_map(|(path, index)| {
                // Skip globals and variables that are local to this block or to one of its children.

                // Skip globals.
                if self.state.symbol_table.lookup_global(&Location::new(self.current_program, path.to_vec())).is_some()
                {
                    return None;
                }

                // Skip variables that are local to this block or to one of its children.
                let local_var_name = *path.last().expect("all paths must have at least one segment.");
                if self.state.symbol_table.is_local_to_or_in_child_scope(input.block.id(), local_var_name) {
                    return None;
                }

                // All other variables become parameters to the async function being built.
                let var = self.state.symbol_table.lookup_local(local_var_name)?;
                Some(make_inputs_and_arguments(self, local_var_name, &var.type_, *index))
            })
            .flatten()
            .unzip();

        // Step 4: Replacement logic used to patch the async block
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

        // Step 5: Reconstruct the block with replaced references
        let mut replacer = Replacer::new(replace_expr, true /* refresh IDs */, self.state);
        let new_block = replacer.reconstruct_block(input.block.clone()).0;

        // Ensure we're not trying to capture too many variables
        if inputs.len() > self.max_inputs {
            self.state.handler.emit_err(leo_errors::StaticAnalyzerError::async_block_capturing_too_many_vars(
                inputs.len(),
                self.max_inputs,
                input.span,
            ));
        }

        // Step 6: Define the new async function
        let function = Function {
            annotations: vec![],
            variant: Variant::AsyncFunction,
            identifier: make_identifier(self, finalize_fn_name),
            const_parameters: vec![],
            input: inputs,
            output: vec![],          // `async function`s can't have returns
            output_type: Type::Unit, // Always the case for `async function`s
            block: new_block,
            span: input.span,
            id: self.state.node_builder.next_id(),
        };

        // Register the generated function
        self.new_async_functions.push((finalize_fn_name, function));

        // Step 7: Create the call expression to invoke the async function
        let call_to_finalize = CallExpression {
            function: Path::new(
                vec![],
                make_identifier(self, finalize_fn_name),
                true,
                Some(vec![finalize_fn_name]), // the finalize function lives in the top level program scope
                Span::default(),
                self.state.node_builder.next_id(),
            ),
            const_arguments: vec![],
            arguments,
            program: Some(self.current_program),
            span: input.span,
            id: self.state.node_builder.next_id(),
        };

        self.modified = true;

        (call_to_finalize.into(), ())
    }

    fn reconstruct_block(&mut self, input: Block) -> (Block, Self::AdditionalOutput) {
        self.in_scope(input.id(), |slf| {
            (
                Block {
                    statements: input.statements.into_iter().map(|s| slf.reconstruct_statement(s).0).collect(),
                    span: input.span,
                    id: input.id,
                },
                Default::default(),
            )
        })
    }

    fn reconstruct_iteration(&mut self, input: IterationStatement) -> (Statement, Self::AdditionalOutput) {
        self.in_scope(input.id(), |slf| {
            (
                IterationStatement {
                    type_: input.type_.map(|ty| slf.reconstruct_type(ty).0),
                    start: slf.reconstruct_expression(input.start, &()).0,
                    stop: slf.reconstruct_expression(input.stop, &()).0,
                    block: slf.reconstruct_block(input.block).0,
                    ..input
                }
                .into(),
                Default::default(),
            )
        })
    }
}
