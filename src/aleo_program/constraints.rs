use crate::aleo_program::{
    BooleanExpression, BooleanSpreadOrExpression, Expression, FieldExpression,
    FieldRangeOrExpression, FieldSpreadOrExpression, Function, Program, Statement, Struct,
    StructMember, Type, Variable,
};

use snarkos_models::curves::{Field, PrimeField};
use snarkos_models::gadgets::utilities::eq::EqGadget;
use snarkos_models::gadgets::{
    r1cs::ConstraintSystem,
    utilities::{alloc::AllocGadget, boolean::Boolean, eq::ConditionalEqGadget, uint32::UInt32},
};
use std::collections::HashMap;
use std::fmt;

#[derive(Clone)]
pub enum ResolvedValue {
    Boolean(Boolean),
    BooleanArray(Vec<Boolean>),
    FieldElement(UInt32),
    FieldElementArray(Vec<UInt32>),
    StructDefinition(Struct),
    StructExpression(Variable, Vec<StructMember>),
    Function(Function),
    Return(Vec<ResolvedValue>), // add Null for function returns
}

impl fmt::Display for ResolvedValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ResolvedValue::Boolean(ref value) => write!(f, "{}", value.get_value().unwrap()),
            ResolvedValue::BooleanArray(ref array) => {
                write!(f, "[")?;
                for (i, e) in array.iter().enumerate() {
                    write!(f, "{}", e.get_value().unwrap())?;
                    if i < array.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, "]")
            }
            ResolvedValue::FieldElement(ref value) => write!(f, "{}", value.value.unwrap()),
            ResolvedValue::FieldElementArray(ref array) => {
                write!(f, "[")?;
                for (i, e) in array.iter().enumerate() {
                    write!(f, "{}", e.value.unwrap())?;
                    if i < array.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, "]")
            }
            ResolvedValue::StructExpression(ref variable, ref members) => {
                write!(f, "{} {{", variable)?;
                for (i, member) in members.iter().enumerate() {
                    write!(f, "{}: {}", member.variable, member.expression)?;
                    if i < members.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, "}}")
            }
            ResolvedValue::Return(ref values) => {
                write!(f, "Return values : [")?;
                for (i, value) in values.iter().enumerate() {
                    write!(f, "{}", value)?;
                    if i < values.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, "]")
            }
            _ => unimplemented!("display not impl for value"),
        }
    }
}

pub struct ResolvedProgram {
    pub resolved_variables: HashMap<Variable, ResolvedValue>,
}

impl ResolvedProgram {
    fn new() -> Self {
        Self {
            resolved_variables: HashMap::new(),
        }
    }

    fn insert(&mut self, variable: Variable, value: ResolvedValue) {
        self.resolved_variables.insert(variable, value);
    }

    fn bool_from_variable<F: Field + PrimeField, CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        variable: Variable,
    ) -> Boolean {
        if self.resolved_variables.contains_key(&variable) {
            // TODO: return synthesis error: "assignment missing" here
            match self.resolved_variables.get(&variable).unwrap() {
                ResolvedValue::Boolean(boolean) => boolean.clone(),
                _ => panic!("expected a boolean, got field"),
            }
        } else {
            let argument = std::env::args()
                .nth(1)
                .unwrap_or("true".into())
                .parse::<bool>()
                .unwrap();
            println!(" argument passed to command line a = {:?}\n", argument);
            // let a = true;
            Boolean::alloc(cs.ns(|| variable.0), || Ok(argument)).unwrap()
        }
    }

    fn u32_from_variable<F: Field + PrimeField, CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        variable: Variable,
    ) -> UInt32 {
        if self.resolved_variables.contains_key(&variable) {
            // TODO: return synthesis error: "assignment missing" here
            match self.resolved_variables.get(&variable).unwrap() {
                ResolvedValue::FieldElement(field) => field.clone(),
                _ => panic!("expected a field, got boolean"),
            }
        } else {
            let argument = std::env::args()
                .nth(1)
                .unwrap_or("1".into())
                .parse::<u32>()
                .unwrap();

            println!(" argument passed to command line a = {:?}\n", argument);

            // let a = 1;
            UInt32::alloc(cs.ns(|| variable.0), Some(argument)).unwrap()
        }
    }

    fn get_bool_value<F: Field + PrimeField, CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        expression: BooleanExpression,
    ) -> Boolean {
        match expression {
            BooleanExpression::Variable(variable) => self.bool_from_variable(cs, variable),
            BooleanExpression::Value(value) => Boolean::Constant(value),
            expression => match self.enforce_boolean_expression(cs, expression) {
                ResolvedValue::Boolean(value) => value,
                _ => unimplemented!("boolean expression did not resolve to boolean"),
            },
        }
    }

    fn get_u32_value<F: Field + PrimeField + PrimeField, CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        expression: FieldExpression,
    ) -> UInt32 {
        match expression {
            FieldExpression::Variable(variable) => self.u32_from_variable(cs, variable),
            FieldExpression::Number(number) => UInt32::constant(number),
            field => match self.enforce_field_expression(cs, field) {
                ResolvedValue::FieldElement(value) => value,
                _ => unimplemented!("field expression did not resolve to field"),
            },
        }
    }

    fn enforce_not<F: Field + PrimeField, CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        expression: BooleanExpression,
    ) -> Boolean {
        let expression = self.get_bool_value(cs, expression);

        expression.not()
    }

    fn enforce_or<F: Field + PrimeField, CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        left: BooleanExpression,
        right: BooleanExpression,
    ) -> Boolean {
        let left = self.get_bool_value(cs, left);
        let right = self.get_bool_value(cs, right);

        Boolean::or(cs, &left, &right).unwrap()
    }

    fn enforce_and<F: Field + PrimeField, CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        left: BooleanExpression,
        right: BooleanExpression,
    ) -> Boolean {
        let left = self.get_bool_value(cs, left);
        let right = self.get_bool_value(cs, right);

        Boolean::and(cs, &left, &right).unwrap()
    }

    fn enforce_bool_equality<F: Field + PrimeField, CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        left: BooleanExpression,
        right: BooleanExpression,
    ) -> Boolean {
        let left = self.get_bool_value(cs, left);
        let right = self.get_bool_value(cs, right);

        left.enforce_equal(cs.ns(|| format!("enforce bool equal")), &right)
            .unwrap();

        Boolean::Constant(true)
    }

    fn enforce_field_equality<F: Field + PrimeField, CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        left: FieldExpression,
        right: FieldExpression,
    ) -> Boolean {
        let left = self.get_u32_value(cs, left);
        let right = self.get_u32_value(cs, right);

        left.conditional_enforce_equal(
            cs.ns(|| format!("enforce field equal")),
            &right,
            &Boolean::Constant(true),
        )
        .unwrap();

        Boolean::Constant(true)
    }

    fn enforce_boolean_expression<F: Field + PrimeField, CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        expression: BooleanExpression,
    ) -> ResolvedValue {
        match expression {
            BooleanExpression::Variable(variable) => {
                ResolvedValue::Boolean(self.bool_from_variable(cs, variable))
            }
            BooleanExpression::Value(value) => ResolvedValue::Boolean(Boolean::Constant(value)),
            BooleanExpression::Not(expression) => {
                ResolvedValue::Boolean(self.enforce_not(cs, *expression))
            }
            BooleanExpression::Or(left, right) => {
                ResolvedValue::Boolean(self.enforce_or(cs, *left, *right))
            }
            BooleanExpression::And(left, right) => {
                ResolvedValue::Boolean(self.enforce_and(cs, *left, *right))
            }
            BooleanExpression::BoolEq(left, right) => {
                ResolvedValue::Boolean(self.enforce_bool_equality(cs, *left, *right))
            }
            BooleanExpression::FieldEq(left, right) => {
                ResolvedValue::Boolean(self.enforce_field_equality(cs, *left, *right))
            }
            BooleanExpression::IfElse(first, second, third) => {
                let resolved_first = match self.enforce_boolean_expression(cs, *first) {
                    ResolvedValue::Boolean(resolved) => resolved,
                    _ => unimplemented!("if else conditional must resolve to boolean"),
                };
                if resolved_first.eq(&Boolean::Constant(true)) {
                    self.enforce_boolean_expression(cs, *second)
                } else {
                    self.enforce_boolean_expression(cs, *third)
                }
            }
            BooleanExpression::Array(array) => ResolvedValue::BooleanArray(
                array
                    .into_iter()
                    .map(|element| match *element {
                        BooleanSpreadOrExpression::Spread(_spread) => {
                            unimplemented!("spreads not enforced yet")
                        }
                        BooleanSpreadOrExpression::BooleanExpression(expression) => {
                            match self.enforce_boolean_expression(cs, expression) {
                                ResolvedValue::Boolean(value) => value,
                                _ => unimplemented!("cannot resolve boolean"),
                            }
                        }
                    })
                    .collect::<Vec<Boolean>>(),
            ),
            _ => unimplemented!(),
        }
    }

    fn enforce_add<F: Field + PrimeField, CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        left: FieldExpression,
        right: FieldExpression,
    ) -> UInt32 {
        let left = self.get_u32_value(cs, left);
        let right = self.get_u32_value(cs, right);

        UInt32::addmany(
            cs.ns(|| format!("enforce {} + {}", left.value.unwrap(), right.value.unwrap())),
            &[left, right],
        )
        .unwrap()
    }

    fn enforce_sub<F: Field + PrimeField, CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        left: FieldExpression,
        right: FieldExpression,
    ) -> UInt32 {
        let left = self.get_u32_value(cs, left);
        let right = self.get_u32_value(cs, right);

        left.sub(
            cs.ns(|| format!("enforce {} - {}", left.value.unwrap(), right.value.unwrap())),
            &right,
        )
        .unwrap()
    }

    fn enforce_mul<F: Field + PrimeField, CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        left: FieldExpression,
        right: FieldExpression,
    ) -> UInt32 {
        let left = self.get_u32_value(cs, left);
        let right = self.get_u32_value(cs, right);

        let res = left
            .mul(
                cs.ns(|| format!("enforce {} * {}", left.value.unwrap(), right.value.unwrap())),
                &right,
            )
            .unwrap();

        res
    }

    fn enforce_div<F: Field + PrimeField, CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        left: FieldExpression,
        right: FieldExpression,
    ) -> UInt32 {
        let left = self.get_u32_value(cs, left);
        let right = self.get_u32_value(cs, right);

        left.div(
            cs.ns(|| format!("enforce {} / {}", left.value.unwrap(), right.value.unwrap())),
            &right,
        )
        .unwrap()
    }

    fn enforce_pow<F: Field + PrimeField, CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        left: FieldExpression,
        right: FieldExpression,
    ) -> UInt32 {
        let left = self.get_u32_value(cs, left);
        let right = self.get_u32_value(cs, right);

        left.pow(
            cs.ns(|| {
                format!(
                    "enforce {} ** {}",
                    left.value.unwrap(),
                    right.value.unwrap()
                )
            }),
            &right,
        )
        .unwrap()
    }

    fn enforce_field_expression<F: Field + PrimeField, CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        expression: FieldExpression,
    ) -> ResolvedValue {
        match expression {
            FieldExpression::Variable(variable) => {
                ResolvedValue::FieldElement(self.u32_from_variable(cs, variable))
            }
            FieldExpression::Number(number) => {
                ResolvedValue::FieldElement(UInt32::constant(number))
            }
            FieldExpression::Add(left, right) => {
                ResolvedValue::FieldElement(self.enforce_add(cs, *left, *right))
            }
            FieldExpression::Sub(left, right) => {
                ResolvedValue::FieldElement(self.enforce_sub(cs, *left, *right))
            }
            FieldExpression::Mul(left, right) => {
                ResolvedValue::FieldElement(self.enforce_mul(cs, *left, *right))
            }
            FieldExpression::Div(left, right) => {
                ResolvedValue::FieldElement(self.enforce_div(cs, *left, *right))
            }
            FieldExpression::Pow(left, right) => {
                ResolvedValue::FieldElement(self.enforce_pow(cs, *left, *right))
            }
            FieldExpression::IfElse(first, second, third) => {
                let resolved_first = match self.enforce_boolean_expression(cs, *first) {
                    ResolvedValue::Boolean(resolved) => resolved,
                    _ => unimplemented!("if else conditional must resolve to boolean"),
                };

                if resolved_first.eq(&Boolean::Constant(true)) {
                    self.enforce_field_expression(cs, *second)
                } else {
                    self.enforce_field_expression(cs, *third)
                }
            }
            FieldExpression::Array(array) => ResolvedValue::FieldElementArray(
                array
                    .into_iter()
                    .map(|element| match *element {
                        FieldSpreadOrExpression::Spread(_spread) => {
                            unimplemented!("spreads not enforced yet")
                        }
                        FieldSpreadOrExpression::FieldExpression(expression) => {
                            match self.enforce_field_expression(cs, expression) {
                                ResolvedValue::FieldElement(value) => value,
                                _ => unimplemented!("cannot resolve field"),
                            }
                        }
                    })
                    .collect::<Vec<UInt32>>(),
            ),
        }
    }

    fn enforce_struct_expression<F: Field + PrimeField, CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        variable: Variable,
        members: Vec<StructMember>,
    ) -> ResolvedValue {
        if let Some(resolved_value) = self.resolved_variables.get_mut(&variable) {
            match resolved_value {
                ResolvedValue::StructDefinition(struct_definition) => {
                    struct_definition
                        .fields
                        .clone()
                        .iter()
                        .zip(members.clone().into_iter())
                        .for_each(|(field, member)| {
                            if field.variable != member.variable {
                                unimplemented!("struct field variables do not match")
                            }
                            // Resolve and possibly enforce struct fields
                            // do we need to store the results here?
                            let _result = self.enforce_expression(cs, member.expression);
                        });

                    ResolvedValue::StructExpression(variable, members)
                }
                _ => unimplemented!("Inline struct type is not defined as a struct"),
            }
        } else {
            unimplemented!("Struct must be declared before it is used in an inline expression")
        }
    }

    fn enforce_index<F: Field + PrimeField, CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        index: FieldExpression,
    ) -> usize {
        match self.enforce_field_expression(cs, index) {
            ResolvedValue::FieldElement(number) => number.value.unwrap() as usize,
            value => unimplemented!("From index must resolve to a uint32, got {}", value),
        }
    }

    fn enforce_array_access_expression<F: Field + PrimeField, CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        array: Box<Expression>,
        index: FieldRangeOrExpression,
    ) -> ResolvedValue {
        match self.enforce_expression(cs, *array) {
            ResolvedValue::FieldElementArray(field_array) => {
                match index {
                    FieldRangeOrExpression::Range(from, to) => {
                        let from_resolved = match from {
                            Some(from_index) => self.enforce_index(cs, from_index),
                            None => 0usize, // Array slice starts at index 0
                        };
                        let to_resolved = match to {
                            Some(to_index) => self.enforce_index(cs, to_index),
                            None => field_array.len(), // Array slice ends at array length
                        };
                        ResolvedValue::FieldElementArray(
                            field_array[from_resolved..to_resolved].to_owned(),
                        )
                    }
                    FieldRangeOrExpression::FieldExpression(index) => {
                        let index_resolved = self.enforce_index(cs, index);
                        ResolvedValue::FieldElement(field_array[index_resolved].to_owned())
                    }
                }
            }
            ResolvedValue::BooleanArray(bool_array) => {
                match index {
                    FieldRangeOrExpression::Range(from, to) => {
                        let from_resolved = match from {
                            Some(from_index) => self.enforce_index(cs, from_index),
                            None => 0usize, // Array slice starts at index 0
                        };
                        let to_resolved = match to {
                            Some(to_index) => self.enforce_index(cs, to_index),
                            None => bool_array.len(), // Array slice ends at array length
                        };
                        ResolvedValue::BooleanArray(
                            bool_array[from_resolved..to_resolved].to_owned(),
                        )
                    }
                    FieldRangeOrExpression::FieldExpression(index) => {
                        let index_resolved = self.enforce_index(cs, index);
                        ResolvedValue::Boolean(bool_array[index_resolved].to_owned())
                    }
                }
            }
            value => unimplemented!("Cannot access element of untyped array {}", value),
        }
    }

    fn enforce_struct_access_expression<F: Field + PrimeField, CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        struct_variable: Box<Expression>,
        struct_member: Variable,
    ) -> ResolvedValue {
        match self.enforce_expression(cs, *struct_variable) {
            ResolvedValue::StructExpression(_name, members) => {
                let matched_member = members
                    .into_iter()
                    .find(|member| member.variable == struct_member);
                match matched_member {
                    Some(member) => self.enforce_expression(cs, member.expression),
                    None => unimplemented!("Cannot access struct member {}", struct_member.0),
                }
            }
            value => unimplemented!("Cannot access element of untyped struct {}", value),
        }
    }

    fn enforce_function_access_expression<F: Field + PrimeField, CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        function: Box<Expression>,
        arguments: Vec<Expression>,
    ) -> ResolvedValue {
        match self.enforce_expression(cs, *function) {
            ResolvedValue::Function(function) => self.enforce_function(cs, function, arguments),
            value => unimplemented!("Cannot call unknown function {}", value),
        }
    }

    fn enforce_expression<F: Field + PrimeField, CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        expression: Expression,
    ) -> ResolvedValue {
        match expression {
            Expression::Boolean(boolean_expression) => {
                self.enforce_boolean_expression(cs, boolean_expression)
            }
            Expression::FieldElement(field_expression) => {
                self.enforce_field_expression(cs, field_expression)
            }
            Expression::Variable(unresolved_variable) => {
                if self.resolved_variables.contains_key(&unresolved_variable) {
                    // Reassigning variable to another variable
                    self.resolved_variables
                        .get_mut(&unresolved_variable)
                        .unwrap()
                        .clone()
                } else {
                    // The type of the unassigned variable depends on what is passed in
                    if std::env::args()
                        .nth(1)
                        .expect("variable declaration not passed in")
                        .parse::<bool>()
                        .is_ok()
                    {
                        ResolvedValue::Boolean(self.bool_from_variable(cs, unresolved_variable))
                    } else {
                        ResolvedValue::FieldElement(self.u32_from_variable(cs, unresolved_variable))
                    }
                }
            }
            Expression::Struct(struct_name, members) => {
                self.enforce_struct_expression(cs, struct_name, members)
            }
            Expression::ArrayAccess(array, index) => {
                self.enforce_array_access_expression(cs, array, index)
            }
            Expression::StructMemberAccess(struct_variable, struct_member) => {
                self.enforce_struct_access_expression(cs, struct_variable, struct_member)
            }
            Expression::FunctionCall(function, arguments) => {
                self.enforce_function_access_expression(cs, function, arguments)
            }
        }
    }

    fn enforce_definition_statement<F: Field + PrimeField, CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        variable: Variable,
        expression: Expression,
    ) {
        let result = self.enforce_expression(cs, expression);
        // println!("  statement result: {} = {}", variable.0, result);
        self.insert(variable, result);
    }

    fn enforce_return_statement<F: Field + PrimeField, CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        statements: Vec<Expression>,
    ) -> ResolvedValue {
        ResolvedValue::Return(
            statements
                .into_iter()
                .map(|expression| match expression {
                    Expression::Boolean(boolean_expression) => {
                        self.enforce_boolean_expression(cs, boolean_expression)
                    }
                    Expression::FieldElement(field_expression) => {
                        self.enforce_field_expression(cs, field_expression)
                    }
                    Expression::Variable(variable) => {
                        self.resolved_variables.get_mut(&variable).unwrap().clone()
                    }
                    Expression::Struct(_v, _m) => {
                        unimplemented!("return struct not impl");
                    }
                    expr => unimplemented!("expression {} can't be returned yet", expr),
                })
                .collect::<Vec<ResolvedValue>>(),
        )
    }

    // fn enforce_statement<F: Field + PrimeField, CS: ConstraintSystem<F>>(
    //     &mut self,
    //     cs: &mut CS,
    //     statement: Statement,
    // ) {
    //     match statement {
    //         Statement::Definition(variable, expression) => {
    //             self.enforce_definition_statement(cs, variable, expression);
    //         }
    //         Statement::Return(statements) => {
    //             let res = self.enforce_return_statement(cs, statements);
    //
    //         }
    //     };
    // }

    fn enforce_function<F: Field + PrimeField, CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        function: Function,
        arguments: Vec<Expression>,
    ) -> ResolvedValue {
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
                // Check visibility here

                // Check that argument is correct type
                match parameter.ty.clone() {
                    Type::FieldElement => {
                        match self.enforce_expression(cs, argument) {
                            ResolvedValue::FieldElement(field) => {
                                // Store argument as variable with parameter name
                                // TODO: this will not support multiple function calls or variables with same name as parameter
                                self.resolved_variables.insert(
                                    parameter.variable.clone(),
                                    ResolvedValue::FieldElement(field),
                                );
                            }
                            argument => unimplemented!("expected field argument got {}", argument),
                        }
                    }
                    Type::Boolean => match self.enforce_expression(cs, argument) {
                        ResolvedValue::Boolean(bool) => {
                            self.resolved_variables
                                .insert(parameter.variable.clone(), ResolvedValue::Boolean(bool));
                        }
                        argument => unimplemented!("expected boolean argument got {}", argument),
                    },
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
                    self.enforce_definition_statement(cs, variable, expression)
                }
                Statement::Return(expressions) => {
                    return_values = self.enforce_return_statement(cs, expressions)
                }
            });

        return_values
    }

    pub fn generate_constraints<F: Field + PrimeField, CS: ConstraintSystem<F>>(
        cs: &mut CS,
        program: Program,
    ) {
        let mut resolved_program = ResolvedProgram::new();

        program
            .structs
            .into_iter()
            .for_each(|(variable, struct_def)| {
                resolved_program
                    .resolved_variables
                    .insert(variable, ResolvedValue::StructDefinition(struct_def));
            });
        program
            .functions
            .into_iter()
            .for_each(|(variable, function)| {
                resolved_program
                    .resolved_variables
                    .insert(variable, ResolvedValue::Function(function));
            });

        let main = resolved_program
            .resolved_variables
            .get(&Variable("main".into()))
            .expect("main function not defined");

        let result = match main.clone() {
            ResolvedValue::Function(function) => {
                resolved_program.enforce_function(cs, function, vec![])
            }
            _ => unimplemented!("main must be a function"),
        };
        println!("\n  {}", result);

        // program
        //     .statements
        //     .into_iter()
        //     .for_each(|statement| resolved_program.enforce_statement(cs, statement));
    }
}

// impl Program {
//     pub fn setup(&self) {
//         self.statements
//             .iter()
//             .for_each(|statement| {
//
//             })
//     }
// }
