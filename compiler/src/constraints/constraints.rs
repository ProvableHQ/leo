//! Methods to enforce constraints and construct a resolved aleo program.

use crate::{
    ast,
    constraints::{
        new_scope, new_scope_from_variable, new_variable_from_variables, ResolvedProgram,
        ResolvedValue,
    },
    types::{Expression, Function, Program, Type},
    Import, ParameterValue,
};

use from_pest::FromPest;
use snarkos_models::curves::{Field, PrimeField};
use snarkos_models::gadgets::r1cs::ConstraintSystem;
use std::fs;
use std::path::Path;

impl<F: Field + PrimeField, CS: ConstraintSystem<F>> ResolvedProgram<F, CS> {
    fn enforce_argument(
        &mut self,
        cs: &mut CS,
        scope: String,
        caller_scope: String,
        function_name: String,
        argument: Expression<F>,
    ) -> ResolvedValue<F> {
        match argument {
            Expression::Variable(variable) => self.enforce_variable(caller_scope, variable),
            expression => self.enforce_expression(cs, scope, function_name, expression),
        }
    }

    pub(crate) fn enforce_function(
        &mut self,
        cs: &mut CS,
        scope: String,
        caller_scope: String,
        function: Function<F>,
        arguments: Vec<Expression<F>>,
    ) -> ResolvedValue<F> {
        let function_name = new_scope(scope.clone(), function.get_name());

        // Make sure we are given the correct number of arguments
        if function.parameters.len() != arguments.len() {
            unimplemented!(
                "function expected {} arguments, got {}",
                function.parameters.len(),
                arguments.len()
            )
        }

        // Store argument values as new variables in resolved program
        function
            .parameters
            .clone()
            .iter()
            .zip(arguments.clone().into_iter())
            .for_each(|(parameter, argument)| {
                // Check that argument is correct type
                match parameter.ty.clone() {
                    Type::U32 => {
                        match self.enforce_argument(
                            cs,
                            scope.clone(),
                            caller_scope.clone(),
                            function_name.clone(),
                            argument,
                        ) {
                            ResolvedValue::U32(number) => {
                                // Store argument as variable with {function_name}_{parameter name}
                                let variable_name = new_scope_from_variable(
                                    function_name.clone(),
                                    &parameter.variable,
                                );
                                self.store(variable_name, ResolvedValue::U32(number));
                            }
                            argument => {
                                unimplemented!("expected integer argument got {}", argument)
                            }
                        }
                    }
                    Type::FieldElement => {
                        match self.enforce_argument(
                            cs,
                            scope.clone(),
                            caller_scope.clone(),
                            function_name.clone(),
                            argument,
                        ) {
                            ResolvedValue::FieldElement(fe) => {
                                // Store argument as variable with {function_name}_{parameter name}
                                let variable_name = new_scope_from_variable(
                                    function_name.clone(),
                                    &parameter.variable,
                                );
                                self.store(variable_name, ResolvedValue::FieldElement(fe));
                            }
                            argument => unimplemented!("expected field argument got {}", argument),
                        }
                    }
                    Type::Boolean => {
                        match self.enforce_argument(
                            cs,
                            scope.clone(),
                            caller_scope.clone(),
                            function_name.clone(),
                            argument,
                        ) {
                            ResolvedValue::Boolean(bool) => {
                                // Store argument as variable with {function_name}_{parameter name}
                                let variable_name = new_scope_from_variable(
                                    function_name.clone(),
                                    &parameter.variable,
                                );
                                self.store(variable_name, ResolvedValue::Boolean(bool));
                            }
                            argument => {
                                unimplemented!("expected boolean argument got {}", argument)
                            }
                        }
                    }
                    ty => unimplemented!("parameter type {} not matched yet", ty),
                }
            });

        // Evaluate function statements

        let mut return_values = ResolvedValue::Return(vec![]);

        for statement in function.statements.iter() {
            if let Some(returned) = self.enforce_statement(
                cs,
                scope.clone(),
                function_name.clone(),
                statement.clone(),
                function.returns.clone(),
            ) {
                return_values = returned;
                break;
            }
        }

        return_values
    }

    fn enforce_main_function(
        &mut self,
        cs: &mut CS,
        scope: String,
        function: Function<F>,
        parameters: Vec<Option<ParameterValue<F>>>,
    ) -> ResolvedValue<F> {
        let function_name = new_scope(scope.clone(), function.get_name());
        let mut arguments = vec![];

        // todo: check parameters length for each

        // Iterate over main function parameters
        function
            .parameters
            .clone()
            .into_iter()
            .zip(parameters.into_iter())
            .for_each(|(parameter_model, parameter_value)| {
                // append each variable to arguments vector
                arguments.push(Expression::Variable(match parameter_model.ty {
                    Type::U32 => self.u32_from_parameter(
                        cs,
                        function_name.clone(),
                        parameter_model,
                        parameter_value,
                    ),
                    Type::FieldElement => self.field_element_from_parameter(
                        cs,
                        function_name.clone(),
                        parameter_model,
                        parameter_value,
                    ),
                    Type::Boolean => self.bool_from_parameter(
                        cs,
                        function_name.clone(),
                        parameter_model,
                        parameter_value,
                    ),
                    Type::Array(ref ty, _length) => match *ty.clone() {
                        Type::U32 => self.u32_array_from_parameter(
                            cs,
                            function_name.clone(),
                            parameter_model,
                            parameter_value,
                        ),
                        Type::FieldElement => self.field_element_array_from_parameter(
                            cs,
                            function_name.clone(),
                            parameter_model,
                            parameter_value,
                        ),
                        Type::Boolean => self.boolean_array_from_parameter(
                            cs,
                            function_name.clone(),
                            parameter_model,
                            parameter_value,
                        ),
                        ty => unimplemented!("parameter type not implemented {}", ty),
                    },
                    ty => unimplemented!("parameter type not implemented {}", ty),
                }))
            });

        self.enforce_function(cs, scope, function_name, function, arguments)
    }

    fn enforce_import(&mut self, cs: &mut CS, scope: String, import: Import<F>) {
        // Resolve program file path
        let unparsed_file = fs::read_to_string(Path::new(&import.path_string_full())).expect(
            &format!("cannot parse import {}", import.path_string_full()),
        );
        let mut file = ast::parse(&unparsed_file).expect(&format!(
            "cannot parse import {}",
            import.path_string_full()
        ));

        // generate ast from file
        let syntax_tree = ast::File::from_pest(&mut file).expect("infallible");

        // generate aleo program from file
        let mut program = Program::from(syntax_tree, import.path_string.clone());

        // Use same namespace as calling function for imported symbols
        program = program.name(scope);

        // * -> import all imports, structs, functions in the current scope
        if import.is_star() {
            // recursively evaluate program statements
            self.resolve_definitions(cs, program);
        } else {
            let program_name = program.name.clone();

            // match each import symbol to a symbol in the imported file
            import.symbols.into_iter().for_each(|symbol| {
                // see if the imported symbol is a struct
                let matched_struct = program
                    .structs
                    .clone()
                    .into_iter()
                    .find(|(struct_name, _struct_def)| symbol.symbol == *struct_name);

                match matched_struct {
                    Some((_struct_name, struct_def)) => {
                        // take the alias if it is present
                        let resolved_name = symbol.alias.unwrap_or(symbol.symbol);
                        let resolved_struct_name =
                            new_variable_from_variables(&program_name.clone(), &resolved_name);

                        // store imported struct under resolved name
                        self.store_variable(
                            resolved_struct_name,
                            ResolvedValue::StructDefinition(struct_def),
                        );
                    }
                    None => {
                        // see if the imported symbol is a function
                        let matched_function = program.functions.clone().into_iter().find(
                            |(function_name, _function)| symbol.symbol.name == *function_name.0,
                        );

                        match matched_function {
                            Some((_function_name, function)) => {
                                // take the alias if it is present
                                let resolved_name = symbol.alias.unwrap_or(symbol.symbol);
                                let resolved_function_name = new_variable_from_variables(
                                    &program_name.clone(),
                                    &resolved_name,
                                );

                                // store imported function under resolved name
                                self.store_variable(
                                    resolved_function_name,
                                    ResolvedValue::Function(function),
                                )
                            }
                            None => unimplemented!(
                                "cannot find imported symbol {} in imported file {}",
                                symbol,
                                program_name.clone()
                            ),
                        }
                    }
                }
            });

            // evaluate all import statements in imported file
            program.imports.into_iter().for_each(|nested_import| {
                self.enforce_import(cs, program_name.name.clone(), nested_import)
            });
        }
    }

    pub fn resolve_definitions(&mut self, cs: &mut CS, program: Program<F>) {
        let program_name = program.name.clone();

        // evaluate and store all imports
        program
            .imports
            .into_iter()
            .for_each(|import| self.enforce_import(cs, program_name.name.clone(), import));

        // evaluate and store all struct definitions
        program
            .structs
            .into_iter()
            .for_each(|(variable, struct_def)| {
                let resolved_struct_name =
                    new_variable_from_variables(&program_name.clone(), &variable);
                self.store_variable(
                    resolved_struct_name,
                    ResolvedValue::StructDefinition(struct_def),
                );
            });

        // evaluate and store all function definitions
        program
            .functions
            .into_iter()
            .for_each(|(function_name, function)| {
                let resolved_function_name = new_scope(program_name.name.clone(), function_name.0);
                self.store(resolved_function_name, ResolvedValue::Function(function));
            });
    }

    pub fn generate_constraints(
        cs: &mut CS,
        program: Program<F>,
        parameters: Vec<Option<ParameterValue<F>>>,
    ) -> ResolvedValue<F> {
        let mut resolved_program = ResolvedProgram::new();
        let program_name = program.get_name();
        let main_function_name = new_scope(program_name.clone(), "main".into());

        resolved_program.resolve_definitions(cs, program);

        let main = resolved_program
            .get(&main_function_name)
            .expect("main function not defined");

        match main.clone() {
            ResolvedValue::Function(function) => {
                let result =
                    resolved_program.enforce_main_function(cs, program_name, function, parameters);
                log::debug!("{}", result);
                result
            }
            _ => unimplemented!("main must be a function"),
        }
    }
}
