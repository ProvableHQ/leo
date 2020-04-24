//! Methods to enforce constraints a resolved aleo program.

use crate::ast;
use crate::constraints::{new_scope_from_variable, ResolvedProgram, ResolvedValue};
use crate::{Expression, Function, Import, Program, Statement, Type};

use from_pest::FromPest;
use snarkos_models::curves::{Field, PrimeField};
use snarkos_models::gadgets::r1cs::ConstraintSystem;
use std::fs;

impl<F: Field + PrimeField, CS: ConstraintSystem<F>> ResolvedProgram<F, CS> {
    pub(crate) fn enforce_function(
        &mut self,
        cs: &mut CS,
        function: Function<F>,
        arguments: Vec<Expression<F>>,
    ) -> ResolvedValue<F> {
        // Make sure we are given the correct number of arguments
        if function.parameters.len() != arguments.len() {
            unimplemented!(
                "function expected {} arguments, got {}",
                function.parameters.len(),
                arguments.len()
            )
        }

        // Store arguments as variables in resolved program
        function
            .parameters
            .clone()
            .iter()
            .zip(arguments.clone().into_iter())
            .for_each(|(parameter, argument)| {
                // Check that argument is correct type
                match parameter.ty.clone() {
                    Type::U32 => {
                        match self.enforce_expression(cs, function.get_name(), argument) {
                            ResolvedValue::U32(number) => {
                                // Store argument as variable with {function_name}_{parameter name}
                                let variable_name = new_scope_from_variable(
                                    function.get_name(),
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
                        match self.enforce_expression(cs, function.get_name(), argument) {
                            ResolvedValue::FieldElement(field) => {
                                // Store argument as variable with {function_name}_{parameter name}
                                let variable_name = new_scope_from_variable(
                                    function.get_name(),
                                    &parameter.variable,
                                );
                                self.store(variable_name, ResolvedValue::FieldElement(field));
                            }
                            argument => unimplemented!("expected field argument got {}", argument),
                        }
                    }
                    Type::Boolean => {
                        match self.enforce_expression(cs, function.get_name(), argument) {
                            ResolvedValue::Boolean(bool) => {
                                // Store argument as variable with {function_name}_{parameter name}
                                let variable_name = new_scope_from_variable(
                                    function.get_name(),
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

        function
            .statements
            .clone()
            .into_iter()
            .for_each(|statement| match statement {
                Statement::Definition(variable, expression) => {
                    self.enforce_definition_statement(
                        cs,
                        function.get_name(),
                        variable,
                        expression,
                    );
                }
                Statement::For(index, start, stop, statements) => {
                    self.enforce_for_statement(
                        cs,
                        function.get_name(),
                        index,
                        start,
                        stop,
                        statements,
                    );
                }
                Statement::Return(expressions) => {
                    return_values = self.enforce_return_statement(
                        cs,
                        function.get_name(),
                        expressions,
                        function.returns.to_owned(),
                    )
                }
            });

        return_values
    }

    fn enforce_main_function(&mut self, cs: &mut CS, function: Function<F>) -> ResolvedValue<F> {
        let mut arguments = vec![];

        // Iterate over main function parameters
        function
            .parameters
            .clone()
            .into_iter()
            .enumerate()
            .for_each(|(i, parameter)| {
                // append each variable to arguments vector
                arguments.push(Expression::Variable(match parameter.ty {
                    Type::U32 => {
                        self.integer_from_parameter(cs, function.get_name(), i + 1, parameter)
                    }
                    Type::FieldElement => {
                        self.field_element_from_parameter(cs, function.get_name(), i + 1, parameter)
                    }
                    Type::Boolean => {
                        self.bool_from_parameter(cs, function.get_name(), i + 1, parameter)
                    }
                    Type::Array(ref ty, _length) => match *ty.clone() {
                        Type::U32 => self.integer_array_from_parameter(
                            cs,
                            function.get_name(),
                            i + 1,
                            parameter,
                        ),
                        Type::FieldElement => self.field_element_array_from_parameter(
                            cs,
                            function.get_name(),
                            i + 1,
                            parameter,
                        ),
                        Type::Boolean => self.boolean_array_from_parameter(
                            cs,
                            function.get_name(),
                            i + 1,
                            parameter,
                        ),
                        ty => unimplemented!("parameter type not implemented {}", ty),
                    },
                    ty => unimplemented!("parameter type not implemented {}", ty),
                }))
            });

        self.enforce_function(cs, function, arguments)
    }

    fn enforce_import(&mut self, cs: &mut CS, import: Import) {
        // Resolve program file path
        let unparsed_file = fs::read_to_string(import.get_file()).expect("cannot read file");
        let mut file = ast::parse(&unparsed_file).expect("unsuccessful parse");

        // generate ast from file
        let syntax_tree = ast::File::from_pest(&mut file).expect("infallible");

        // generate aleo program from file
        let program = Program::from(syntax_tree);

        // recursively evaluate program statements TODO: in file scope
        self.resolve_definitions(cs, program);

        // store import under designated name
        // self.store(name, value)
    }

    pub fn resolve_definitions(&mut self, cs: &mut CS, program: Program<F>) {
        program
            .imports
            .into_iter()
            .for_each(|import| self.enforce_import(cs, import));
        program
            .structs
            .into_iter()
            .for_each(|(variable, struct_def)| {
                self.store_variable(variable, ResolvedValue::StructDefinition(struct_def));
            });
        program
            .functions
            .into_iter()
            .for_each(|(function_name, function)| {
                self.store(function_name.0, ResolvedValue::Function(function));
            });
    }

    pub fn generate_constraints(cs: &mut CS, program: Program<F>) {
        let mut resolved_program = ResolvedProgram::new();

        resolved_program.resolve_definitions(cs, program);

        let main = resolved_program
            .get(&"main".into())
            .expect("main function not defined");

        let result = match main.clone() {
            ResolvedValue::Function(function) => {
                resolved_program.enforce_main_function(cs, function)
            }
            _ => unimplemented!("main must be a function"),
        };
        println!("\n  {}", result);
    }
}
