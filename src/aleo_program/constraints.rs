use crate::aleo_program::{
    Assignee, BooleanExpression, BooleanSpreadOrExpression, Expression, Function, Import, Integer,
    IntegerExpression, IntegerRangeOrExpression, IntegerSpreadOrExpression, Program, Statement,
    Struct, StructMember, Type, Variable,
};
use crate::ast;

use from_pest::FromPest;
use snarkos_models::curves::{Field, PrimeField};
use snarkos_models::gadgets::utilities::eq::EqGadget;
use snarkos_models::gadgets::{
    r1cs::ConstraintSystem,
    utilities::{alloc::AllocGadget, boolean::Boolean, eq::ConditionalEqGadget, uint32::UInt32},
};
use std::collections::HashMap;
use std::fmt;
use std::fs;
use std::marker::PhantomData;

#[derive(Clone)]
pub enum ResolvedValue<F: Field + PrimeField> {
    Boolean(Boolean),
    BooleanArray(Vec<Boolean>),
    U32(UInt32),
    U32Array(Vec<UInt32>),
    StructDefinition(Struct<F>),
    StructExpression(Variable<F>, Vec<StructMember<F>>),
    Function(Function<F>),
    Return(Vec<ResolvedValue<F>>), // add Null for function returns
}

impl<F: Field + PrimeField> fmt::Display for ResolvedValue<F> {
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
            ResolvedValue::U32(ref value) => write!(f, "{}", value.value.unwrap()),
            ResolvedValue::U32Array(ref array) => {
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
            ResolvedValue::StructDefinition(ref _definition) => {
                unimplemented!("cannot return struct definition in program")
            }
            ResolvedValue::Function(ref _function) => {
                unimplemented!("cannot return function definition in program")
            } // _ => unimplemented!("display not impl for value"),
        }
    }
}

pub struct ResolvedProgram<F: Field + PrimeField, CS: ConstraintSystem<F>> {
    pub resolved_names: HashMap<String, ResolvedValue<F>>,
    pub _cs: PhantomData<CS>,
}

pub fn new_scope(outer: String, inner: String) -> String {
    format!("{}_{}", outer, inner)
}

pub fn new_scope_from_variable<F: Field + PrimeField>(
    outer: String,
    inner: &Variable<F>,
) -> String {
    new_scope(outer, inner.name.clone())
}

impl<F: Field + PrimeField, CS: ConstraintSystem<F>> ResolvedProgram<F, CS> {
    fn new() -> Self {
        Self {
            resolved_names: HashMap::new(),
            _cs: PhantomData::<CS>,
        }
    }

    fn store(&mut self, name: String, value: ResolvedValue<F>) {
        self.resolved_names.insert(name, value);
    }

    fn store_variable(&mut self, variable: Variable<F>, value: ResolvedValue<F>) {
        self.store(variable.name, value);
    }

    fn contains_name(&self, name: &String) -> bool {
        self.resolved_names.contains_key(name)
    }

    fn contains_variable(&self, variable: &Variable<F>) -> bool {
        self.contains_name(&variable.name)
    }

    fn get(&self, name: &String) -> Option<&ResolvedValue<F>> {
        self.resolved_names.get(name)
    }

    fn get_mut(&mut self, name: &String) -> Option<&mut ResolvedValue<F>> {
        self.resolved_names.get_mut(name)
    }

    fn get_mut_variable(&mut self, variable: &Variable<F>) -> Option<&mut ResolvedValue<F>> {
        self.get_mut(&variable.name)
    }

    /// Constrain integers

    fn integer_from_variable(
        &mut self,
        cs: &mut CS,
        scope: String,
        variable: Variable<F>,
    ) -> ResolvedValue<F> {
        // Evaluate variable name in current function scope
        let variable_name = new_scope_from_variable(scope, &variable);

        if self.contains_name(&variable_name) {
            // TODO: return synthesis error: "assignment missing" here
            self.get(&variable_name).unwrap().clone()
        } else {
            // TODO: remove this after resolving arguments
            let argument = std::env::args()
                .nth(1)
                .unwrap_or("1".into())
                .parse::<u32>()
                .unwrap();

            println!(" argument passed to command line a = {:?}\n", argument);

            // let a = 1;
            ResolvedValue::U32(UInt32::alloc(cs.ns(|| variable.name), Some(argument)).unwrap())
        }
    }

    fn get_integer_constant(integer: Integer) -> ResolvedValue<F> {
        match integer {
            Integer::U32(u32_value) => ResolvedValue::U32(UInt32::constant(u32_value)),
        }
    }

    fn get_integer_value(
        &mut self,
        cs: &mut CS,
        scope: String,
        expression: IntegerExpression<F>,
    ) -> ResolvedValue<F> {
        match expression {
            IntegerExpression::Variable(variable) => {
                self.integer_from_variable(cs, scope, variable)
            }
            IntegerExpression::Number(number) => Self::get_integer_constant(number),
            field => self.enforce_integer_expression(cs, scope, field),
        }
    }

    fn enforce_u32_equality(cs: &mut CS, left: UInt32, right: UInt32) -> Boolean {
        left.conditional_enforce_equal(
            cs.ns(|| format!("enforce field equal")),
            &right,
            &Boolean::Constant(true),
        )
        .unwrap();

        Boolean::Constant(true)
    }

    fn enforce_integer_equality(
        &mut self,
        cs: &mut CS,
        scope: String,
        left: IntegerExpression<F>,
        right: IntegerExpression<F>,
    ) -> Boolean {
        let left = self.get_integer_value(cs, scope.clone(), left);
        let right = self.get_integer_value(cs, scope.clone(), right);

        match (left, right) {
            (ResolvedValue::U32(left_u32), ResolvedValue::U32(right_u32)) => {
                Self::enforce_u32_equality(cs, left_u32, right_u32)
            }
            (left_int, right_int) => {
                unimplemented!("equality not impl between {} == {}", left_int, right_int)
            }
        }
    }

    fn enforce_u32_add(cs: &mut CS, left: UInt32, right: UInt32) -> ResolvedValue<F> {
        ResolvedValue::U32(
            UInt32::addmany(
                cs.ns(|| format!("enforce {} + {}", left.value.unwrap(), right.value.unwrap())),
                &[left, right],
            )
            .unwrap(),
        )
    }

    fn enforce_add(
        &mut self,
        cs: &mut CS,
        scope: String,
        left: IntegerExpression<F>,
        right: IntegerExpression<F>,
    ) -> ResolvedValue<F> {
        let left = self.get_integer_value(cs, scope.clone(), left);
        let right = self.get_integer_value(cs, scope.clone(), right);

        match (left, right) {
            (ResolvedValue::U32(left_u32), ResolvedValue::U32(right_u32)) => {
                Self::enforce_u32_add(cs, left_u32, right_u32)
            }
            (left_int, right_int) => {
                unimplemented!("add not impl between {} + {}", left_int, right_int)
            }
        }
    }

    fn enforce_u32_sub(cs: &mut CS, left: UInt32, right: UInt32) -> ResolvedValue<F> {
        ResolvedValue::U32(
            left.sub(
                cs.ns(|| format!("enforce {} - {}", left.value.unwrap(), right.value.unwrap())),
                &right,
            )
            .unwrap(),
        )
    }

    fn enforce_sub(
        &mut self,
        cs: &mut CS,
        scope: String,
        left: IntegerExpression<F>,
        right: IntegerExpression<F>,
    ) -> ResolvedValue<F> {
        let left = self.get_integer_value(cs, scope.clone(), left);
        let right = self.get_integer_value(cs, scope.clone(), right);

        match (left, right) {
            (ResolvedValue::U32(left_u32), ResolvedValue::U32(right_u32)) => {
                Self::enforce_u32_sub(cs, left_u32, right_u32)
            }
            (left_int, right_int) => {
                unimplemented!("add not impl between {} + {}", left_int, right_int)
            }
        }
    }

    fn enforce_u32_mul(cs: &mut CS, left: UInt32, right: UInt32) -> ResolvedValue<F> {
        ResolvedValue::U32(
            left.mul(
                cs.ns(|| format!("enforce {} * {}", left.value.unwrap(), right.value.unwrap())),
                &right,
            )
            .unwrap(),
        )
    }

    fn enforce_mul(
        &mut self,
        cs: &mut CS,
        scope: String,
        left: IntegerExpression<F>,
        right: IntegerExpression<F>,
    ) -> ResolvedValue<F> {
        let left = self.get_integer_value(cs, scope.clone(), left);
        let right = self.get_integer_value(cs, scope.clone(), right);

        match (left, right) {
            (ResolvedValue::U32(left_u32), ResolvedValue::U32(right_u32)) => {
                Self::enforce_u32_mul(cs, left_u32, right_u32)
            }
            (left_int, right_int) => {
                unimplemented!("add not impl between {} + {}", left_int, right_int)
            }
        }
    }

    fn enforce_u32_div(cs: &mut CS, left: UInt32, right: UInt32) -> ResolvedValue<F> {
        ResolvedValue::U32(
            left.div(
                cs.ns(|| format!("enforce {} / {}", left.value.unwrap(), right.value.unwrap())),
                &right,
            )
            .unwrap(),
        )
    }

    fn enforce_div(
        &mut self,
        cs: &mut CS,
        scope: String,
        left: IntegerExpression<F>,
        right: IntegerExpression<F>,
    ) -> ResolvedValue<F> {
        let left = self.get_integer_value(cs, scope.clone(), left);
        let right = self.get_integer_value(cs, scope.clone(), right);

        match (left, right) {
            (ResolvedValue::U32(left_u32), ResolvedValue::U32(right_u32)) => {
                Self::enforce_u32_div(cs, left_u32, right_u32)
            }
            (left_int, right_int) => {
                unimplemented!("add not impl between {} + {}", left_int, right_int)
            }
        }
    }

    fn enforce_u32_pow(cs: &mut CS, left: UInt32, right: UInt32) -> ResolvedValue<F> {
        ResolvedValue::U32(
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
            .unwrap(),
        )
    }

    fn enforce_pow(
        &mut self,
        cs: &mut CS,
        scope: String,
        left: IntegerExpression<F>,
        right: IntegerExpression<F>,
    ) -> ResolvedValue<F> {
        let left = self.get_integer_value(cs, scope.clone(), left);
        let right = self.get_integer_value(cs, scope.clone(), right);

        match (left, right) {
            (ResolvedValue::U32(left_u32), ResolvedValue::U32(right_u32)) => {
                Self::enforce_u32_pow(cs, left_u32, right_u32)
            }
            (left_int, right_int) => {
                unimplemented!("add not impl between {} + {}", left_int, right_int)
            }
        }
    }

    fn enforce_integer_expression(
        &mut self,
        cs: &mut CS,
        scope: String,
        expression: IntegerExpression<F>,
    ) -> ResolvedValue<F> {
        match expression {
            IntegerExpression::Variable(variable) => {
                self.integer_from_variable(cs, scope, variable)
            }
            IntegerExpression::Number(number) => Self::get_integer_constant(number),
            IntegerExpression::Add(left, right) => self.enforce_add(cs, scope, *left, *right),
            IntegerExpression::Sub(left, right) => self.enforce_sub(cs, scope, *left, *right),
            IntegerExpression::Mul(left, right) => self.enforce_mul(cs, scope, *left, *right),
            IntegerExpression::Div(left, right) => self.enforce_div(cs, scope, *left, *right),
            IntegerExpression::Pow(left, right) => self.enforce_pow(cs, scope, *left, *right),
            IntegerExpression::IfElse(first, second, third) => {
                let resolved_first =
                    match self.enforce_boolean_expression(cs, scope.clone(), *first) {
                        ResolvedValue::Boolean(resolved) => resolved,
                        _ => unimplemented!("if else conditional must resolve to boolean"),
                    };

                if resolved_first.eq(&Boolean::Constant(true)) {
                    self.enforce_integer_expression(cs, scope, *second)
                } else {
                    self.enforce_integer_expression(cs, scope, *third)
                }
            }
            IntegerExpression::Array(array) => {
                let mut result = vec![];
                array.into_iter().for_each(|element| match *element {
                    IntegerSpreadOrExpression::Spread(spread) => match spread {
                        IntegerExpression::Variable(variable) => {
                            let array_name = new_scope_from_variable(scope.clone(), &variable);
                            match self.get(&array_name) {
                                Some(value) => match value {
                                    ResolvedValue::U32Array(array) => result.extend(array.clone()),
                                    value => unimplemented!(
                                        "spreads only implemented for arrays, got {}",
                                        value
                                    ),
                                },
                                None => unimplemented!(
                                    "cannot copy elements from array that does not exist {}",
                                    variable.name
                                ),
                            }
                        }
                        value => {
                            unimplemented!("spreads only implemented for arrays, got {}", value)
                        }
                    },
                    IntegerSpreadOrExpression::Expression(expression) => {
                        match self.enforce_integer_expression(cs, scope.clone(), expression) {
                            ResolvedValue::U32(value) => result.push(value),
                            _ => unimplemented!("cannot resolve field"),
                        }
                    }
                });
                ResolvedValue::U32Array(result)
            }
        }
    }

    fn bool_from_variable(&mut self, cs: &mut CS, scope: String, variable: Variable<F>) -> Boolean {
        // Evaluate variable name in current function scope
        let variable_name = new_scope_from_variable(scope, &variable);

        if self.contains_name(&variable_name) {
            // TODO: return synthesis error: "assignment missing" here
            match self.get(&variable_name).unwrap() {
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
            Boolean::alloc(cs.ns(|| variable.name), || Ok(argument)).unwrap()
        }
    }

    fn get_bool_value(
        &mut self,
        cs: &mut CS,
        scope: String,
        expression: BooleanExpression<F>,
    ) -> Boolean {
        match expression {
            BooleanExpression::Variable(variable) => self.bool_from_variable(cs, scope, variable),
            BooleanExpression::Value(value) => Boolean::Constant(value),
            expression => match self.enforce_boolean_expression(cs, scope, expression) {
                ResolvedValue::Boolean(value) => value,
                _ => unimplemented!("boolean expression did not resolve to boolean"),
            },
        }
    }

    fn enforce_not(
        &mut self,
        cs: &mut CS,
        scope: String,
        expression: BooleanExpression<F>,
    ) -> Boolean {
        let expression = self.get_bool_value(cs, scope, expression);

        expression.not()
    }

    fn enforce_or(
        &mut self,
        cs: &mut CS,
        scope: String,
        left: BooleanExpression<F>,
        right: BooleanExpression<F>,
    ) -> Boolean {
        let left = self.get_bool_value(cs, scope.clone(), left);
        let right = self.get_bool_value(cs, scope.clone(), right);

        Boolean::or(cs, &left, &right).unwrap()
    }

    fn enforce_and(
        &mut self,
        cs: &mut CS,
        scope: String,
        left: BooleanExpression<F>,
        right: BooleanExpression<F>,
    ) -> Boolean {
        let left = self.get_bool_value(cs, scope.clone(), left);
        let right = self.get_bool_value(cs, scope.clone(), right);

        Boolean::and(cs, &left, &right).unwrap()
    }

    fn enforce_bool_equality(
        &mut self,
        cs: &mut CS,
        scope: String,
        left: BooleanExpression<F>,
        right: BooleanExpression<F>,
    ) -> Boolean {
        let left = self.get_bool_value(cs, scope.clone(), left);
        let right = self.get_bool_value(cs, scope.clone(), right);

        left.enforce_equal(cs.ns(|| format!("enforce bool equal")), &right)
            .unwrap();

        Boolean::Constant(true)
    }

    fn enforce_boolean_expression(
        &mut self,
        cs: &mut CS,
        scope: String,
        expression: BooleanExpression<F>,
    ) -> ResolvedValue<F> {
        match expression {
            BooleanExpression::Variable(variable) => {
                ResolvedValue::Boolean(self.bool_from_variable(cs, scope, variable))
            }
            BooleanExpression::Value(value) => ResolvedValue::Boolean(Boolean::Constant(value)),
            BooleanExpression::Not(expression) => {
                ResolvedValue::Boolean(self.enforce_not(cs, scope, *expression))
            }
            BooleanExpression::Or(left, right) => {
                ResolvedValue::Boolean(self.enforce_or(cs, scope, *left, *right))
            }
            BooleanExpression::And(left, right) => {
                ResolvedValue::Boolean(self.enforce_and(cs, scope, *left, *right))
            }
            BooleanExpression::BoolEq(left, right) => {
                ResolvedValue::Boolean(self.enforce_bool_equality(cs, scope, *left, *right))
            }
            BooleanExpression::FieldEq(left, right) => {
                ResolvedValue::Boolean(self.enforce_integer_equality(cs, scope, *left, *right))
            }
            BooleanExpression::IfElse(first, second, third) => {
                let resolved_first =
                    match self.enforce_boolean_expression(cs, scope.clone(), *first) {
                        ResolvedValue::Boolean(resolved) => resolved,
                        _ => unimplemented!("if else conditional must resolve to boolean"),
                    };
                if resolved_first.eq(&Boolean::Constant(true)) {
                    self.enforce_boolean_expression(cs, scope, *second)
                } else {
                    self.enforce_boolean_expression(cs, scope, *third)
                }
            }
            BooleanExpression::Array(array) => {
                let mut result = vec![];
                array.into_iter().for_each(|element| match *element {
                    BooleanSpreadOrExpression::Spread(spread) => match spread {
                        BooleanExpression::Variable(variable) => {
                            let array_name = new_scope_from_variable(scope.clone(), &variable);
                            match self.get(&array_name) {
                                Some(value) => match value {
                                    ResolvedValue::BooleanArray(array) => {
                                        result.extend(array.clone())
                                    }
                                    value => unimplemented!(
                                        "spreads only implemented for arrays, got {}",
                                        value
                                    ),
                                },
                                None => unimplemented!(
                                    "cannot copy elements from array that does not exist {}",
                                    variable.name
                                ),
                            }
                        }
                        value => {
                            unimplemented!("spreads only implemented for arrays, got {}", value)
                        }
                    },
                    BooleanSpreadOrExpression::Expression(expression) => {
                        match self.enforce_boolean_expression(cs, scope.clone(), expression) {
                            ResolvedValue::Boolean(value) => result.push(value),
                            value => {
                                unimplemented!("expected boolean for boolean array, got {}", value)
                            }
                        }
                    }
                });
                ResolvedValue::BooleanArray(result)
            }
            expression => unimplemented!("boolean expression {}", expression),
        }
    }

    fn enforce_struct_expression(
        &mut self,
        cs: &mut CS,
        scope: String,
        variable: Variable<F>,
        members: Vec<StructMember<F>>,
    ) -> ResolvedValue<F> {
        if let Some(resolved_value) = self.get_mut_variable(&variable) {
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
                            let _result =
                                self.enforce_expression(cs, scope.clone(), member.expression);
                        });

                    ResolvedValue::StructExpression(variable, members)
                }
                _ => unimplemented!("Inline struct type is not defined as a struct"),
            }
        } else {
            unimplemented!("Struct must be declared before it is used in an inline expression")
        }
    }

    fn enforce_index(&mut self, cs: &mut CS, scope: String, index: IntegerExpression<F>) -> usize {
        match self.enforce_integer_expression(cs, scope.clone(), index) {
            ResolvedValue::U32(number) => number.value.unwrap() as usize,
            value => unimplemented!("From index must resolve to a uint32, got {}", value),
        }
    }

    fn enforce_array_access_expression(
        &mut self,
        cs: &mut CS,
        scope: String,
        array: Box<Expression<F>>,
        index: IntegerRangeOrExpression<F>,
    ) -> ResolvedValue<F> {
        match self.enforce_expression(cs, scope.clone(), *array) {
            ResolvedValue::U32Array(field_array) => {
                match index {
                    IntegerRangeOrExpression::Range(from, to) => {
                        let from_resolved = match from {
                            Some(from_index) => self.enforce_index(cs, scope.clone(), from_index),
                            None => 0usize, // Array slice starts at index 0
                        };
                        let to_resolved = match to {
                            Some(to_index) => self.enforce_index(cs, scope.clone(), to_index),
                            None => field_array.len(), // Array slice ends at array length
                        };
                        ResolvedValue::U32Array(field_array[from_resolved..to_resolved].to_owned())
                    }
                    IntegerRangeOrExpression::Expression(index) => {
                        let index_resolved = self.enforce_index(cs, scope.clone(), index);
                        ResolvedValue::U32(field_array[index_resolved].to_owned())
                    }
                }
            }
            ResolvedValue::BooleanArray(bool_array) => {
                match index {
                    IntegerRangeOrExpression::Range(from, to) => {
                        let from_resolved = match from {
                            Some(from_index) => self.enforce_index(cs, scope.clone(), from_index),
                            None => 0usize, // Array slice starts at index 0
                        };
                        let to_resolved = match to {
                            Some(to_index) => self.enforce_index(cs, scope.clone(), to_index),
                            None => bool_array.len(), // Array slice ends at array length
                        };
                        ResolvedValue::BooleanArray(
                            bool_array[from_resolved..to_resolved].to_owned(),
                        )
                    }
                    IntegerRangeOrExpression::Expression(index) => {
                        let index_resolved = self.enforce_index(cs, scope.clone(), index);
                        ResolvedValue::Boolean(bool_array[index_resolved].to_owned())
                    }
                }
            }
            value => unimplemented!("Cannot access element of untyped array {}", value),
        }
    }

    fn enforce_struct_access_expression(
        &mut self,
        cs: &mut CS,
        scope: String,
        struct_variable: Box<Expression<F>>,
        struct_member: Variable<F>,
    ) -> ResolvedValue<F> {
        match self.enforce_expression(cs, scope.clone(), *struct_variable) {
            ResolvedValue::StructExpression(_name, members) => {
                let matched_member = members
                    .into_iter()
                    .find(|member| member.variable == struct_member);
                match matched_member {
                    Some(member) => self.enforce_expression(cs, scope.clone(), member.expression),
                    None => unimplemented!("Cannot access struct member {}", struct_member.name),
                }
            }
            value => unimplemented!("Cannot access element of untyped struct {}", value),
        }
    }

    fn enforce_function_access_expression(
        &mut self,
        cs: &mut CS,
        scope: String,
        function: Box<Expression<F>>,
        arguments: Vec<Expression<F>>,
    ) -> ResolvedValue<F> {
        match self.enforce_expression(cs, scope, *function) {
            ResolvedValue::Function(function) => self.enforce_function(cs, function, arguments),
            value => unimplemented!("Cannot call unknown function {}", value),
        }
    }

    fn enforce_expression(
        &mut self,
        cs: &mut CS,
        scope: String,
        expression: Expression<F>,
    ) -> ResolvedValue<F> {
        match expression {
            Expression::Boolean(boolean_expression) => {
                self.enforce_boolean_expression(cs, scope, boolean_expression)
            }
            Expression::Integer(field_expression) => {
                self.enforce_integer_expression(cs, scope, field_expression)
            }
            Expression::Variable(unresolved_variable) => {
                let variable_name = new_scope_from_variable(scope, &unresolved_variable);

                // Evaluate the variable name in the current function scope
                if self.contains_name(&variable_name) {
                    // Reassigning variable to another variable
                    self.get_mut(&variable_name).unwrap().clone()
                } else if self.contains_variable(&unresolved_variable) {
                    // Check global scope (function and struct names)
                    self.get_mut_variable(&unresolved_variable).unwrap().clone()
                } else {
                    // The type of the unassigned variable depends on what is passed in
                    if std::env::args()
                        .nth(1)
                        .expect("variable declaration not passed in")
                        .parse::<bool>()
                        .is_ok()
                    {
                        ResolvedValue::Boolean(self.bool_from_variable(
                            cs,
                            variable_name,
                            unresolved_variable,
                        ))
                    } else {
                        self.integer_from_variable(cs, variable_name, unresolved_variable)
                    }
                }
            }
            Expression::Struct(struct_name, members) => {
                self.enforce_struct_expression(cs, scope, struct_name, members)
            }
            Expression::ArrayAccess(array, index) => {
                self.enforce_array_access_expression(cs, scope, array, index)
            }
            Expression::StructMemberAccess(struct_variable, struct_member) => {
                self.enforce_struct_access_expression(cs, scope, struct_variable, struct_member)
            }
            Expression::FunctionCall(function, arguments) => {
                self.enforce_function_access_expression(cs, scope, function, arguments)
            }
            expression => unimplemented!("expression not impl {}", expression),
        }
    }

    fn resolve_assignee(&mut self, scope: String, assignee: Assignee<F>) -> String {
        match assignee {
            Assignee::Variable(name) => new_scope_from_variable(scope, &name),
            Assignee::Array(array, _index) => self.resolve_assignee(scope, *array),
            Assignee::StructMember(struct_variable, _member) => {
                self.resolve_assignee(scope, *struct_variable)
            }
        }
    }

    fn enforce_definition_statement(
        &mut self,
        cs: &mut CS,
        scope: String,
        assignee: Assignee<F>,
        expression: Expression<F>,
    ) {
        // Create or modify the lhs variable in the current function scope
        match assignee {
            Assignee::Variable(name) => {
                // Store the variable in the current scope
                let definition_name = new_scope_from_variable(scope.clone(), &name);

                // Evaluate the rhs expression in the current function scope
                let result = self.enforce_expression(cs, scope, expression);

                self.store(definition_name, result);
            }
            Assignee::Array(array, index_expression) => {
                // Evaluate the rhs expression in the current function scope
                let result = &mut self.enforce_expression(cs, scope.clone(), expression);

                // Check that array exists
                let expected_array_name = self.resolve_assignee(scope.clone(), *array);

                // Resolve index so we know if we are assigning to a single value or a range of values
                match index_expression {
                    IntegerRangeOrExpression::Expression(index) => {
                        let index = self.enforce_index(cs, scope.clone(), index);

                        // Modify the single value of the array in place
                        match self.get_mut(&expected_array_name) {
                            Some(value) => match (value, result) {
                                (ResolvedValue::U32Array(old), ResolvedValue::U32(new)) => {
                                    old[index] = new.to_owned();
                                }
                                (ResolvedValue::BooleanArray(old), ResolvedValue::Boolean(new)) => {
                                    old[index] = new.to_owned();
                                }
                                _ => {
                                    unimplemented!("Cannot assign single index to array of values ")
                                }
                            },
                            None => unimplemented!(
                                "tried to assign to unknown array {}",
                                expected_array_name
                            ),
                        }
                    }
                    IntegerRangeOrExpression::Range(from, to) => {
                        let from_index = match from {
                            Some(expression) => self.enforce_index(cs, scope.clone(), expression),
                            None => 0usize,
                        };
                        let to_index_option = match to {
                            Some(expression) => {
                                Some(self.enforce_index(cs, scope.clone(), expression))
                            }
                            None => None,
                        };

                        // Modify the range of values of the array in place
                        match self.get_mut(&expected_array_name) {
                            Some(value) => match (value, result) {
                                (ResolvedValue::U32Array(old), ResolvedValue::U32Array(new)) => {
                                    let to_index = to_index_option.unwrap_or(old.len());
                                    old.splice(from_index..to_index, new.iter().cloned());
                                }
                                (
                                    ResolvedValue::BooleanArray(old),
                                    ResolvedValue::BooleanArray(new),
                                ) => {
                                    let to_index = to_index_option.unwrap_or(old.len());
                                    old.splice(from_index..to_index, new.iter().cloned());
                                }
                                _ => unimplemented!(
                                    "Cannot assign a range of array values to single value"
                                ),
                            },
                            None => unimplemented!(
                                "tried to assign to unknown array {}",
                                expected_array_name
                            ),
                        }
                    }
                }
            }
            Assignee::StructMember(struct_variable, struct_member) => {
                // Check that struct exists
                let expected_struct_name = self.resolve_assignee(scope.clone(), *struct_variable);

                match self.get_mut(&expected_struct_name) {
                    Some(value) => match value {
                        ResolvedValue::StructExpression(_variable, members) => {
                            // Modify the struct member in place
                            let matched_member = members
                                .into_iter()
                                .find(|member| member.variable == struct_member);
                            match matched_member {
                                Some(mut member) => member.expression = expression,
                                None => unimplemented!(
                                    "struct member {} does not exist in {}",
                                    struct_member,
                                    expected_struct_name
                                ),
                            }
                        }
                        _ => unimplemented!(
                            "tried to assign to unknown struct {}",
                            expected_struct_name
                        ),
                    },
                    None => {
                        unimplemented!("tried to assign to unknown struct {}", expected_struct_name)
                    }
                }
            }
        };
    }

    fn enforce_return_statement(
        &mut self,
        cs: &mut CS,
        scope: String,
        statements: Vec<Expression<F>>,
    ) -> ResolvedValue<F> {
        ResolvedValue::Return(
            statements
                .into_iter()
                .map(|expression| self.enforce_expression(cs, scope.clone(), expression))
                .collect::<Vec<ResolvedValue<F>>>(),
        )
    }

    fn enforce_statement(&mut self, cs: &mut CS, scope: String, statement: Statement<F>) {
        match statement {
            Statement::Definition(variable, expression) => {
                self.enforce_definition_statement(cs, scope, variable, expression);
            }
            Statement::For(index, start, stop, statements) => {
                self.enforce_for_statement(cs, scope, index, start, stop, statements);
            }
            Statement::Return(statements) => {
                // TODO: add support for early termination
                let _res = self.enforce_return_statement(cs, scope, statements);
            }
        };
    }

    fn enforce_for_statement(
        &mut self,
        cs: &mut CS,
        scope: String,
        index: Variable<F>,
        start: IntegerExpression<F>,
        stop: IntegerExpression<F>,
        statements: Vec<Statement<F>>,
    ) {
        let start_index = self.enforce_index(cs, scope.clone(), start);
        let stop_index = self.enforce_index(cs, scope.clone(), stop);

        for i in start_index..stop_index {
            // Store index in current function scope.
            // For loop scope is not implemented.
            let index_name = new_scope_from_variable(scope.clone(), &index);
            self.store(index_name, ResolvedValue::U32(UInt32::constant(i as u32)));

            // Evaluate statements
            statements
                .clone()
                .into_iter()
                .for_each(|statement| self.enforce_statement(cs, scope.clone(), statement));
        }
    }

    fn enforce_function(
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
                // Check visibility here

                // Check that argument is correct type
                match parameter.ty.clone() {
                    Type::U32 => {
                        match self.enforce_expression(cs, function.get_name(), argument) {
                            ResolvedValue::U32(field) => {
                                // Store argument as variable with {function_name}_{parameter name}
                                let variable_name = new_scope_from_variable(
                                    function.get_name(),
                                    &parameter.variable,
                                );
                                self.store(variable_name, ResolvedValue::U32(field));
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
                    return_values =
                        self.enforce_return_statement(cs, function.get_name(), expressions)
                }
            });

        return_values
    }

    fn enforce_import(&mut self, cs: &mut CS, import: Import) {
        // println!("import: {}", import);

        // Resolve program file path
        let unparsed_file = fs::read_to_string(import.get_file()).expect("cannot read file");
        let mut file = ast::parse(&unparsed_file).expect("unsuccessful parse");
        // println!("successful import parse!");

        // generate ast from file
        let syntax_tree = ast::File::from_pest(&mut file).expect("infallible");

        // generate aleo program from file
        let program = Program::from(syntax_tree);
        // println!(" compiled: {:#?}", program);

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
                resolved_program.enforce_function(cs, function, vec![])
            }
            _ => unimplemented!("main must be a function"),
        };
        println!("\n  {}", result);
    }
}
