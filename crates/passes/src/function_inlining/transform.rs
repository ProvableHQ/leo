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

use crate::{
    CompilerState,
    Replacer,
    SsaFormingInput,
    common::{items_at_path, program_functions, stub_functions},
    static_single_assignment::visitor::SsaFormingVisitor,
};
use indexmap::{IndexMap, IndexSet};
use itertools::Itertools;
use leo_ast::{Function, Location, *};
use leo_errors::TypeCheckerWarning;
use leo_span::{Symbol, sym};

pub struct TransformVisitor<'a> {
    pub state: &'a mut CompilerState,
    /// Functions that should always be inlined.
    pub always_inline: IndexSet<Vec<Symbol>>,
    /// A map of reconstructed functions, keyed by `Location` for O(1) lookup during inlining.
    pub reconstructed_functions: IndexMap<Location, Function>,
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
        let top_level_program = input.program_id.as_symbol();
        self.program = top_level_program;

        // DFS starts from every function in `function_map` (across all programs) and follows
        // call edges through program boundaries, producing a post-order that places callees
        // before callers. Transitively-reached nodes that aren't in `function_map` (e.g.
        // `FromAleo` stub entry points) stay in the order and are skipped by `shift_remove`
        // below. The unwrap is safe: type checking guarantees the call graph is acyclic.
        let order =
            self.state.call_graph.post_order_with_filter(|location| self.function_map.contains_key(location)).unwrap();

        // `self.program` is set per-function so `reconstruct_call` can see cross-program calls
        // from the callee's perspective.
        for function_location in order {
            self.program = function_location.program;
            if let Some(function) = self.function_map.shift_remove(&function_location) {
                let reconstructed_function = self.reconstruct_function(function);
                self.reconstructed_functions.insert(function_location, reconstructed_function);
            }
        }

        self.program = top_level_program;

        // Unprocessed external functions are carried through for stub assembly; current-program
        // leftovers are dead code and intentionally dropped.
        for (loc, f) in std::mem::take(&mut self.function_map) {
            if loc.program != self.program {
                self.reconstructed_functions.entry(loc).or_insert(f);
            }
        }

        // The constructor is reconstructed after functions because it may call inlined ones.
        let constructor = input.constructor.map(|constructor| self.reconstruct_constructor(constructor));

        // Split current-program top-level functions off for this scope; keep the rest in
        // `reconstructed_functions` so stub assembly can pick them up.
        let all_reconstructed = core::mem::take(&mut self.reconstructed_functions);
        let mut functions = Vec::new();
        for (loc, f) in all_reconstructed {
            if loc.program != self.program {
                self.reconstructed_functions.insert(loc, f);
                continue;
            }
            // Module functions have been inlined at their call sites and must not appear as
            // standalone functions, so emit only single-segment paths.
            if let Some((last, rest)) = loc.path.split_last()
                && rest.is_empty()
            {
                functions.push((*last, f));
            }
        }

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
        // Seed `function_map` with every function definition reachable from this program (stubs,
        // libraries, and the current program). Then a single DFS over the call graph processes
        // all of them in post-order; recursive per-stub passes are unnecessary. Current-program
        // inserts come last so they override any stub placeholders.
        self.program =
            *input.program_scopes.first().expect("a program must have a single program scope at this stage").0;

        for (_, stub) in &input.stubs {
            for (loc, f) in stub_functions(stub) {
                self.function_map.entry(loc).or_insert_with(|| f.clone());
            }
        }
        for (loc, f) in program_functions(&input) {
            self.function_map.insert(loc, f.clone());
        }

        let program_scopes =
            input.program_scopes.into_iter().map(|(id, scope)| (id, self.reconstruct_program_scope(scope))).collect();

        // Reassemble `FromLeo` stubs directly from `reconstructed_functions`. FromAleo and
        // FromLibrary stubs have nothing to inline here — their functions are either already
        // in Aleo bytecode or have been inlined at their call sites.
        let stubs = input
            .stubs
            .into_iter()
            .map(|(name, stub)| match stub {
                Stub::FromLeo { program, parents } => {
                    (name, Stub::FromLeo { program: self.assemble_from_leo_program(program), parents })
                }
                other @ (Stub::FromAleo { .. } | Stub::FromLibrary { .. }) => (name, other),
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
        let function_location = input.function.expect_global_location();

        // Cross-program entry-point calls are always emitted as direct Aleo `call`s — inlining
        // an entry-point body into a different program would lose its transition semantics.
        if self.state.symbol_table.is_cross_program_entry(self.program, function_location) {
            return (input.into(), Default::default());
        }

        // Post-order traversal guarantees the callee is already reconstructed.
        let callee = self
            .reconstructed_functions
            .get(function_location)
            .expect("guaranteed to exist due to post-order traversal of the call graph.");

        let call_count_ref = self.state.call_count.get_mut(function_location).expect("Guaranteed by type checking");

        let has_no_inline_annotation = callee.annotations.iter().any(|a| a.identifier.name == sym::no_inline);

        // Mandatory inlining conditions
        let mandatory_cond = |cond: bool, msg: &str| -> bool {
            if cond && has_no_inline_annotation {
                self.state.handler.emit_warning(TypeCheckerWarning::no_inline_ignored(
                    callee.identifier.name,
                    msg,
                    callee.annotations.iter().find(|a| a.identifier.name == sym::no_inline).unwrap().span,
                ));
            }
            cond
        };

        let optional_cond = |cond: bool| -> bool { !has_no_inline_annotation && cond };

        // Submodule functions are always inlined: Aleo resources are flat identifiers, so there
        // is no bytecode representation for `path::nested::fn`. This applies identically to
        // current-program and cross-program submodule callees.
        let should_inline = mandatory_cond(function_location.path.len() > 1, "this is a module function")
            || match callee.variant {
                // Always inline library functions (they cannot exist as standalone Aleo functions).
                _ if self.state.symbol_table.is_library(function_location.program) => {
                    mandatory_cond(true, "this is a library function")
                }
                Variant::FinalFn => mandatory_cond(true, "this is a final fn"),
                Variant::Fn => {
                    mandatory_cond(
                    self.is_onchain,
                    "the function is called from an on-chain context (constructor or finalize)",
                ) ||
                mandatory_cond(callee.input.len() > 16, "this function has more than 16 arguments") ||
                mandatory_cond(
                    Self::names_optional_type(&callee.output_type),
                    "this function returns a type naming an optional",
                ) ||
                mandatory_cond(
                    callee.input.iter().any(|arg| Self::names_optional_type(&arg.type_)),
                    "this function has an argument naming an optional",
                ) ||
                mandatory_cond(
                    self.always_inline.contains(&vec![callee.identifier.name]),
                    "this function has been called from another function",
                ) ||
                // Called only once
                optional_cond(*call_count_ref == 1) ||
                // Has no arguments
                optional_cond(callee.input.is_empty()) ||
                // Has only empty arguments
                optional_cond(callee.input.iter().all(|arg| arg.type_.is_empty()))
                }
                Variant::EntryPoint | Variant::Finalize => false,
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

// Private helpers for stub assembly.
impl TransformVisitor<'_> {
    /// Assembles a `FromLeo` stub's program from `reconstructed_functions`. `input.stubs` is
    /// always empty on a stub's program (only the top-level `Program` carries stubs), so it
    /// passes through unchanged.
    fn assemble_from_leo_program(&self, input: Program) -> Program {
        let program_scopes =
            input.program_scopes.into_iter().map(|(id, scope)| (id, self.assemble_from_leo_scope(id, scope))).collect();
        let modules = input.modules.into_iter().map(|(mid, m)| (mid, self.assemble_module(m))).collect();
        Program { program_scopes, modules, stubs: input.stubs, imports: input.imports }
    }

    /// Assembles a single ProgramScope for a FromLeo stub from reconstructed_functions.
    fn assemble_from_leo_scope(&self, program_name: Symbol, input: ProgramScope) -> ProgramScope {
        // Entry-point functions must appear before finalize functions so the type checker
        // can populate async_function_callers before visiting finalizers. Top-level closures
        // stay in the stub — same-program callers (in the stub's own entry points) and cross-
        // program callers (in the compilation unit) both emit direct `call`s into them, so
        // removing them would leave dangling references in the emitted bytecode.
        let (entry_points, non_entry_points): (Vec<_>, Vec<_>) =
            items_at_path(&self.reconstructed_functions, program_name, &[]).partition(|(_, f)| f.variant.is_entry());
        let functions: Vec<_> = entry_points.into_iter().chain(non_entry_points).collect();

        ProgramScope {
            program_id: input.program_id,
            parents: input.parents,
            composites: input.composites,
            mappings: input.mappings,
            storage_variables: input.storage_variables,
            functions,
            interfaces: input.interfaces,
            constructor: input.constructor,
            consts: input.consts,
            span: input.span,
        }
    }

    /// Assembles a Module for a FromLeo stub from reconstructed_functions.
    fn assemble_module(&self, input: Module) -> Module {
        Module {
            functions: items_at_path(&self.reconstructed_functions, input.program_name, &input.path).collect(),
            ..input
        }
    }
}
