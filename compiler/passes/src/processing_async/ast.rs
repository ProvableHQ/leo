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
use itertools::Itertools;
use leo_ast::{
    AstReconstructor,
    AstVisitor,
    AsyncExpression,
    Block,
    CallExpression,
    Composite,
    Expression,
    Function,
    Identifier,
    Input,
    IterationStatement,
    Location,
    Member,
    MemberAccess,
    Mode,
    Node,
    Path,
    ProgramVisitor,
    Statement,
    StructExpression,
    StructVariableInitializer,
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

/// Bundle inputs into structs when they exceed max_inputs.
/// Returns: (new_inputs, new_arguments, synthetic_structs, replacements_for_bundled)
/// where replacements_for_bundled maps (Symbol, Option<usize>) to the path to access the field in the bundled struct.
fn bundle_inputs_into_structs(
    inputs: Vec<Input>,
    arguments: Vec<Expression>,
    input_metadata: Vec<(Symbol, Option<usize>)>,
    max_inputs: usize,
    function_name: Symbol,
    node_builder: &mut leo_ast::NodeBuilder,
    assigner: &mut crate::Assigner,
) -> (Vec<Input>, Vec<Expression>, Vec<Composite>, IndexMap<(Symbol, Option<usize>), Expression>) {
    if inputs.len() <= max_inputs {
        return (inputs, arguments, vec![], IndexMap::new());
    }

    let mut new_inputs = Vec::new();
    let mut new_arguments = Vec::new();
    let mut synthetic_structs = Vec::new();
    let mut replacements = IndexMap::new();

    let total_inputs = inputs.len();
    let bundle_capacity = max_inputs; // 16 is max
    let num_bundles = (total_inputs - max_inputs + bundle_capacity - 2) / (bundle_capacity - 1);
    let unbundled_count = max_inputs - num_bundles;
    
    // Keep the first unbundled_count inputs as-is
    for (input, arg) in inputs.iter().zip(arguments.iter()).take(unbundled_count) {
        new_inputs.push(input.clone());
        new_arguments.push(arg.clone());
    }

    // Bundle remaining inputs into struct(s)
    let remaining_inputs: Vec<_> = inputs.into_iter().skip(unbundled_count).collect();
    let remaining_arguments: Vec<_> = arguments.into_iter().skip(unbundled_count).collect();
    let remaining_metadata: Vec<_> = input_metadata.into_iter().skip(unbundled_count).collect();

    for ((chunk_inputs, chunk_args), chunk_metadata) in remaining_inputs
        .chunks(bundle_capacity)
        .zip(remaining_arguments.chunks(bundle_capacity))
        .zip(remaining_metadata.chunks(bundle_capacity))
    {
        // Generate a unique struct name
        let sanitized_fn_name = Symbol::intern(&function_name.to_string().replace('$', "_"));
        let struct_name = assigner.unique_symbol(sanitized_fn_name, "_bundle");
        
        let struct_identifier = Identifier {
            name: struct_name,
            span: Span::default(),
            id: node_builder.next_id(),
        };

        // Create structt members from inputs
        let members: Vec<Member> = chunk_inputs
            .iter()
            .enumerate()
            .map(|(i, input)| Member {
                mode: Mode::None,
                identifier: Identifier {
                    name: Symbol::intern(&format!("field_{}", i)),
                    span: Span::default(),
                    id: node_builder.next_id(),
                },
                type_: input.type_.clone(),
                span: Span::default(),
                id: node_builder.next_id(),
            })
            .collect();

        // Create the synthetic struct definition
        let composite = Composite {
            identifier: struct_identifier,
            const_parameters: vec![],
            members: members.clone(),
            external: None,
            is_record: false,
            span: Span::default(),
            id: node_builder.next_id(),
        };

        synthetic_structs.push(composite);

        // Create an input parameter for this bundled struct
        // Note: program should be None for local structs - the program context comes from scope during type checking
        let composite_type = Type::Composite(leo_ast::CompositeType {
            path: Path::from(struct_identifier).into_absolute(),
            const_arguments: vec![],
            program: None,
        });
        
        let param_name = assigner.unique_symbol(struct_name, "_param");
        
        let bundle_input = Input {
            identifier: Identifier {
                name: param_name,
                span: Span::default(),
                id: node_builder.next_id(),
            },
            mode: Mode::None,
            type_: composite_type,
            span: Span::default(),
            id: node_builder.next_id(),
        };

        new_inputs.push(bundle_input);

        // Create a struct initialization expression as the argument
        let struct_members: Vec<StructVariableInitializer> = chunk_inputs
            .iter()
            .zip(chunk_args.iter())
            .enumerate()
            .map(|(i, (_, arg))| StructVariableInitializer {
                identifier: Identifier {
                    name: Symbol::intern(&format!("field_{}", i)),
                    span: Span::default(),
                    id: node_builder.next_id(),
                },
                expression: Some(arg.clone()),
                span: Span::default(),
                id: node_builder.next_id(),
            })
            .collect();

        let struct_init = StructExpression {
            path: Path::from(struct_identifier).into_absolute(),
            const_arguments: vec![],
            members: struct_members,
            span: Span::default(),
            id: node_builder.next_id(),
        };

        new_arguments.push(struct_init.into());

        // Track replacements, the original input names should now be accessed via struct.field_N
        for (i, (original_symbol, original_index)) in chunk_metadata.iter().enumerate() {
            // Create a member access expression: param_name.field_i
            let member_access = MemberAccess {
                inner: Path::from(Identifier {
                    name: param_name,
                    span: Span::default(),
                    id: node_builder.next_id(),
                })
                .into(),
                name: Identifier {
                    name: Symbol::intern(&format!("field_{}", i)),
                    span: Span::default(),
                    id: node_builder.next_id(),
                },
                span: Span::default(),
                id: node_builder.next_id(),
            }
            .into();

            replacements.insert((*original_symbol, *original_index), member_access);
        }
    }

    (new_inputs, new_arguments, synthetic_structs, replacements)
}

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

                        replacements.insert((symbol, Some(index)), Path::from(identifier).into());

                        vec![(
                            input,
                            TupleAccess {
                                tuple: Path::from(make_identifier(slf, symbol)).into(),
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

                                let expr: Expression = Path::from(identifier).into();

                                replacements.insert(key, expr.clone());
                                tuple_elements.push(expr.clone());
                                inputs_and_arguments.push((
                                    input,
                                    TupleAccess {
                                        tuple: Path::from(make_identifier(slf, symbol)).into(),
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

                            replacements.insert((symbol, None), Path::from(identifier).into());

                            let argument = Path::from(make_identifier(slf, symbol)).into();
                            vec![(input, argument)]
                        }
                    },
                }
            };

        // Step 3: Resolve symbol accesses into inputs and call arguments
        let inputs_and_args_with_metadata: Vec<_> = access_collector
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
                let inputs_and_args = make_inputs_and_arguments(self, local_var_name, &var.type_, *index);

                // For each (Input, Expression) pair, attach metadata (symbol, index)
                Some(
                    inputs_and_args
                        .into_iter()
                        .map(|(input, arg)| (input, arg, (local_var_name, *index)))
                        .collect::<Vec<_>>()
                )
            })
            .flatten()
            .collect();

        // Separate into parallel vectors
        let (inputs, arguments, input_metadata): (Vec<_>, Vec<_>, Vec<_>) = 
            inputs_and_args_with_metadata.into_iter()
                .map(|(input, arg, metadata)| (input, arg, metadata))
                .multiunzip();

        // Step 4: Bundle inputs if necessary
        let (final_inputs, final_arguments, synthetic_structs, bundle_replacements) = bundle_inputs_into_structs(
            inputs,
            arguments,
            input_metadata,
            self.max_inputs,
            finalize_fn_name,
            &mut self.state.node_builder,
            &mut self.state.assigner,
        );

        // If there are bundle replacements, merge them into the main replacements map
        for (key, expr) in bundle_replacements {
            replacements.insert(key, expr);
        }

        // Step 5: Reconstruct the block with replaced references
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

        let mut replacer = Replacer::new(replace_expr, true /* refresh IDs */, self.state);
        let new_block = replacer.reconstruct_block(input.block.clone()).0;

        // Register synthetic structs
        for composite in synthetic_structs {
            let struct_name = composite.name();
            self.synthetic_structs.push((struct_name, composite));
        }

        // Step 6: Define the new async function
        let function = Function {
            annotations: vec![],
            variant: Variant::AsyncFunction,
            identifier: make_identifier(self, finalize_fn_name),
            const_parameters: vec![],
            input: final_inputs.clone(),
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
            arguments: final_arguments,
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
