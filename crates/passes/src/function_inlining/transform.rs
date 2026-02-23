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

use crate::{CompilerState, Replacer, SsaFormingInput, static_single_assignment::visitor::SsaFormingVisitor};
use indexmap::{IndexMap, IndexSet};
use itertools::Itertools;
use leo_ast::{Function, Location, *};
use leo_span::{Symbol, sym};

pub struct TransformVisitor<'a> {
    pub state: &'a mut CompilerState,
    /// Functions that should always be inlined.
    pub always_inline: IndexSet<Vec<Symbol>>,
    /// A map of reconstructed functions in the current program scope.
    pub reconstructed_functions: Vec<(Location, Function)>,
    /// The main program.
    pub program: Symbol,
    /// A map to provide faster lookup of functions.
    pub function_map: IndexMap<Location, Function>,
    /// Whether or not we are currently traversing a block that's executed onchain (either final block or final fn block).
    pub is_onchain: bool,
}

impl<'a> TransformVisitor<'a> {
    /// Check if a names an optional type. We don't need to check the type
    /// recursively with the sumbol table, hiding optionals behind structs is
    /// allowed.
    fn names_optional_type(ty: &Type) -> bool {
        match ty {
            Type::Optional(_) => true,
            Type::Tuple(tuple) => tuple.elements().iter().any(Self::names_optional_type),
            Type::Array(array) => Self::names_optional_type(array.element_type()),
            _ => false,
        }
    }
}

impl ProgramReconstructor for TransformVisitor<'_> {
    fn reconstruct_program_scope(&mut self, input: ProgramScope) -> ProgramScope {
        // Set the program name.
        self.program = input.program_id.name.name;

        // Get the post-order ordering of the call graph.
        // Note that the post-order always contains all nodes in the call graph.
        // Note that the unwrap is safe since type checking guarantees that the call graph is acyclic.
        // Keep full Location (program + path) to ensure correct lookups.
        let order: Vec<Location> = self
            .state
            .call_graph
            .post_order_with_filter(|location| location.program == self.program)
            .unwrap()
            .into_iter()
            .collect();

        // Reconstruct and accumulate each of the functions in post-order.
        for function_location in order {
            // None: If `function_location` is not in `function_map`, then it must be an external function.
            // TODO: Check that this is indeed an external function. Requires a redesign of the symbol table.
            if let Some(function) = self.function_map.shift_remove(&function_location) {
                // Reconstruct the function.
                let reconstructed_function = self.reconstruct_function(function);
                // Add the reconstructed function to the mapping.
                self.reconstructed_functions.push((function_location.clone(), reconstructed_function));
            }
        }

        // This is a sanity check to ensure that functions in the program scope have been processed.
        assert!(self.function_map.is_empty(), "All functions in the program should have been processed.");

        // Reconstruct the constructor.
        // Note: This must be done after the functions have been reconstructed to ensure that every callee function has been inlined.
        let constructor = input.constructor.map(|constructor| self.reconstruct_constructor(constructor));

        // Note that this intentionally clears `self.reconstructed_functions` for the next program scope.
        let functions = core::mem::take(&mut self.reconstructed_functions)
            .iter()
            .filter_map(|(loc, f)| {
                // Only consider functions defined at program scope. The rest are not relevant since they should all
                // have been inlined by now.
                loc.path.split_last().filter(|(_, rest)| rest.is_empty()).map(|(last, _)| (*last, f.clone()))
            })
            .collect();

        ProgramScope {
            program_id: input.program_id,
            parents: input.parents.into_iter().map(|(s, t)| (s, self.reconstruct_type(t).0)).collect(),
            composites: input.composites,
            mappings: input.mappings,
            storage_variables: input.storage_variables,
            constructor,
            functions,
            interfaces: input.interfaces,
            consts: input.consts,
            span: input.span,
        }
    }

    fn reconstruct_function(&mut self, input: Function) -> Function {
        Function {
            annotations: input.annotations,
            variant: input.variant,
            identifier: input.identifier,
            const_parameters: input.const_parameters,
            input: input.input,
            output: input.output,
            output_type: input.output_type,
            block: {
                // Set the `is_onchain` flag before reconstructing the block.
                self.is_onchain = input.variant.is_onchain();
                // Reconstruct the block.
                let block = self.reconstruct_block(input.block).0;
                // Reset the `is_onchain` flag.
                self.is_onchain = false;
                block
            },
            span: input.span,
            id: input.id,
        }
    }

    fn reconstruct_constructor(&mut self, input: Constructor) -> Constructor {
        Constructor {
            annotations: input.annotations,
            block: {
                // Set the `is_onchain` flag before reconstructing the block.
                self.is_onchain = true;
                // Reconstruct the block.
                let block = self.reconstruct_block(input.block).0;
                // Reset the `is_onchain` flag.
                self.is_onchain = false;
                block
            },
            span: input.span,
            id: input.id,
        }
    }

    fn reconstruct_program(&mut self, input: Program) -> Program {
        // Populate `self.function_map` using the functions in the program scopes and the modules
        // Use full Location (program + path) as keys to avoid collisions between programs
        input
            .modules
            .iter()
            .flat_map(|(module_path, m)| {
                let program = m.program_name;
                m.functions.iter().map(move |(name, f)| {
                    let path: Vec<Symbol> = module_path.iter().cloned().chain(std::iter::once(*name)).collect();
                    (Location::new(program, path), f.clone())
                })
            })
            .chain(input.program_scopes.iter().flat_map(|(program_name, scope)| {
                scope.functions.iter().map(move |(name, f)| (Location::new(*program_name, vec![*name]), f.clone()))
            }))
            .for_each(|(location, f)| {
                self.function_map.insert(location, f);
            });

        // Reconstruct program scopes. Inline functions defined in modules will be traversed
        // using the call graph and reconstructed in the right order.
        let program_scopes =
            input.program_scopes.into_iter().map(|(id, scope)| (id, self.reconstruct_program_scope(scope))).collect();

        // Process FromLeo stubs so that FinalFn functions are inlined within them.
        // This is necessary because code generation will emit bytecode for FromLeo stubs,
        // and FinalFn functions cannot exist as standalone functions in bytecode.
        let stubs = input
            .stubs
            .into_iter()
            .map(|(name, stub)| {
                match stub {
                    Stub::FromLeo { program, parents } => {
                        let processed_program = self.reconstruct_program(program);
                        (name, Stub::FromLeo { program: processed_program, parents })
                    }
                    // FromAleo stubs don't have Leo AST, so nothing to inline
                    other @ Stub::FromAleo { .. } => (name, other),
                }
            })
            .collect();

        Program { program_scopes, stubs, ..input }
    }
}

impl AstReconstructor for TransformVisitor<'_> {
    type AdditionalInput = ();
    type AdditionalOutput = Vec<Statement>;

    /* Expressions */
    fn reconstruct_call(&mut self, input: CallExpression, _additional: &()) -> (Expression, Self::AdditionalOutput) {
        // Type checking guarantees that only functions local to the program scope can be inlined.
        if input.function.expect_global_location().program != self.program {
            return (input.into(), Default::default());
        }

        // Lookup the reconstructed callee function.
        // Since this pass processes functions in post-order, the callee function is guaranteed to exist in `self.reconstructed_functions`
        // Use full Location (program + path) to ensure correct lookup across multiple programs.
        let function_location = input.function.expect_global_location();
        let (_, callee) = self
            .reconstructed_functions
            .iter()
            .find(|(loc, _)| loc == function_location)
            .expect("guaranteed to exist due to post-order traversal of the call graph.");

        let call_count_ref = self.state.call_count.get_mut(function_location).expect("Guaranteed by type checking");

        // TODO: improve inline heuristic
        let should_inline = match callee.variant {
            // Always inline final fns (they cannot exist as standalone functions in code generation)
            Variant::FinalFn => true,
            // Always inline module functions
            _ if function_location.program == self.program && function_location.path.len() > 1 => true,

            // Respect @no_inline for other variants
            _ if callee.annotations.iter().any(|a| a.identifier.name == sym::no_inline) => false,

            Variant::Fn if
                // Called only once
                *call_count_ref == 1 ||
                // Called from onchain context
                self.is_onchain ||
                // Has const parameters
                !callee.const_parameters.is_empty() ||
                // Has no arguments
                callee.input.is_empty() ||
                // Has only empty arguments
                callee.input.iter().all(|arg| arg.type_.is_empty()) ||
                // Has more than 16 arguments
                callee.input.len() > 16 ||
                // Returns a type naming an optional
                Self::names_optional_type(&callee.output_type) ||
                // Has an argument naming an optional
                callee.input.iter().any(|arg| Self::names_optional_type(&arg.type_)) ||
                // Marked by the analysis phase
                self.always_inline.contains(&vec![callee.identifier.name]) => true,
            _ => false,
        };

        // Inline the callee function, if required, otherwise, return the call expression.
        if should_inline {
            // We are inlining, thus removing one call
            *call_count_ref -= 1;

            // Construct a mapping from input variables of the callee function to arguments passed to the callee.
            let parameter_to_argument = callee
                .input
                .iter()
                .map(|input| input.identifier().name)
                .zip_eq(input.arguments)
                .collect::<IndexMap<_, _>>();

            // Function to replace path expressions with their corresponding const argument or keep them unchanged.
            let replace_path = |expr: &Expression| match expr {
                Expression::Path(path) => parameter_to_argument
                    .get(&path.identifier().name)
                    .map_or(Expression::Path(path.clone()), |expr| expr.clone()),
                _ => expr.clone(),
            };

            // Replace path expressions with their corresponding const argument or keep them unchanged.
            let reconstructed_block = Replacer::new(replace_path, false /* refresh IDs */, self.state)
                .reconstruct_block(callee.block.clone())
                .0;

            // Run SSA formation on the inlined block and rename definitions. Renaming is necessary to avoid shadowing variables.
            let mut inlined_statements =
                SsaFormingVisitor::new(self.state, SsaFormingInput { rename_defs: true }, self.program)
                    .consume_block(reconstructed_block);

            // If the inlined block returns a value, then use the value in place of the call expression; otherwise, use the unit expression.
            let result = match inlined_statements.last() {
                Some(Statement::Return(_)) => {
                    // Note that this unwrap is safe since we know that the last statement is a return statement.
                    match inlined_statements.pop().unwrap() {
                        Statement::Return(ReturnStatement { expression, .. }) => expression,
                        _ => panic!("This branch checks that the last statement is a return statement."),
                    }
                }
                _ => {
                    let id = self.state.node_builder.next_id();
                    self.state.type_table.insert(id, Type::Unit);
                    UnitExpression { span: Default::default(), id }.into()
                }
            };

            (result, inlined_statements)
        } else {
            (input.into(), Default::default())
        }
    }

    /* Statements */
    fn reconstruct_assign(&mut self, _input: AssignStatement) -> (Statement, Self::AdditionalOutput) {
        panic!("`AssignStatement`s should not exist in the AST at this phase of compilation.")
    }

    /// Reconstructs the statements inside a basic block, accumulating any statements produced by function inlining.
    fn reconstruct_block(&mut self, block: Block) -> (Block, Self::AdditionalOutput) {
        let mut statements = Vec::with_capacity(block.statements.len());

        for statement in block.statements {
            let (reconstructed_statement, additional_statements) = self.reconstruct_statement(statement);
            statements.extend(additional_statements);
            statements.push(reconstructed_statement);
        }

        (Block { span: block.span, statements, id: block.id }, Default::default())
    }

    /// Flattening removes conditional statements from the program.
    fn reconstruct_conditional(&mut self, input: ConditionalStatement) -> (Statement, Self::AdditionalOutput) {
        if !self.is_onchain {
            panic!("`ConditionalStatement`s should not be in the AST at this phase of compilation.")
        } else {
            (
                ConditionalStatement {
                    condition: self.reconstruct_expression(input.condition, &()).0,
                    then: self.reconstruct_block(input.then).0,
                    otherwise: input.otherwise.map(|n| Box::new(self.reconstruct_statement(*n).0)),
                    span: input.span,
                    id: input.id,
                }
                .into(),
                Default::default(),
            )
        }
    }

    /// Reconstruct a definition statement by inlining any function calls.
    /// This function also segments tuple assignment statements into multiple assignment statements.
    fn reconstruct_definition(&mut self, mut input: DefinitionStatement) -> (Statement, Self::AdditionalOutput) {
        let (value, mut statements) = self.reconstruct_expression(input.value, &());
        match (input.place, value) {
            // If we just inlined the production of a tuple literal, we need multiple definition statements.
            (DefinitionPlace::Multiple(left), Expression::Tuple(right)) => {
                assert_eq!(left.len(), right.elements.len());
                for (identifier, rhs_value) in left.into_iter().zip(right.elements) {
                    let stmt = DefinitionStatement {
                        place: DefinitionPlace::Single(identifier),
                        type_: None,
                        value: rhs_value,
                        span: Default::default(),
                        id: self.state.node_builder.next_id(),
                    }
                    .into();

                    statements.push(stmt);
                }
                (Statement::dummy(), statements)
            }

            (place, value) => {
                input.value = value;
                input.place = place;
                (input.into(), statements)
            }
        }
    }

    /// Reconstructs expression statements by inlining any function calls.
    fn reconstruct_expression_statement(&mut self, input: ExpressionStatement) -> (Statement, Self::AdditionalOutput) {
        // Reconstruct the expression.
        // Note that type checking guarantees that the expression is a function call.
        let (expression, additional_statements) = self.reconstruct_expression(input.expression, &());

        // If the resulting expression is a unit expression, return a dummy statement.
        let statement = match expression {
            Expression::Unit(_) => Statement::dummy(),
            _ => ExpressionStatement { expression, ..input }.into(),
        };

        (statement, additional_statements)
    }

    /// Loop unrolling unrolls and removes iteration statements from the program.
    fn reconstruct_iteration(&mut self, _: IterationStatement) -> (Statement, Self::AdditionalOutput) {
        panic!("`IterationStatement`s should not be in the AST at this phase of compilation.");
    }
}
