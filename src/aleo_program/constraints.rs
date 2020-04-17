use crate::aleo_program::{
    Assignee, BooleanExpression, BooleanSpreadOrExpression, Expression, FieldExpression,
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
            ResolvedValue::StructDefinition(ref _definition) => {
                unimplemented!("cannot return struct definition in program")
            }
            ResolvedValue::Function(ref _function) => {
                unimplemented!("cannot return function definition in program")
            } // _ => unimplemented!("display not impl for value"),
        }
    }
}

pub struct ResolvedProgram {
    pub resolved_names: HashMap<String, ResolvedValue>,
}

pub fn new_scope(outer: String, inner: String) -> String {
    format!("{}_{}", outer, inner)
}

pub fn new_scope_from_variable(outer: String, inner: &Variable) -> String {
    new_scope(outer, inner.0.clone())
}

impl ResolvedProgram {
    fn new() -> Self {
        Self {
            resolved_names: HashMap::new(),
        }
    }

    fn store(&mut self, name: String, value: ResolvedValue) {
        self.resolved_names.insert(name, value);
    }

    fn store_variable(&mut self, variable: Variable, value: ResolvedValue) {
        self.store(variable.0, value);
    }

    fn contains_name(&self, name: &String) -> bool {
        self.resolved_names.contains_key(name)
    }

    fn contains_variable(&self, variable: &Variable) -> bool {
        self.contains_name(&variable.0)
    }

    fn get(&self, name: &String) -> Option<&ResolvedValue> {
        self.resolved_names.get(name)
    }

    fn get_mut(&mut self, name: &String) -> Option<&mut ResolvedValue> {
        self.resolved_names.get_mut(name)
    }

    fn get_mut_variable(&mut self, variable: &Variable) -> Option<&mut ResolvedValue> {
        self.get_mut(&variable.0)
    }

    fn bool_from_variable<F: Field + PrimeField, CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        scope: String,
        variable: Variable,
    ) -> Boolean {
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
            Boolean::alloc(cs.ns(|| variable.0), || Ok(argument)).unwrap()
        }
    }

    fn u32_from_variable<F: Field + PrimeField, CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        scope: String,
        variable: Variable,
    ) -> UInt32 {
        // Evaluate variable name in current function scope
        let variable_name = new_scope_from_variable(scope, &variable);

        if self.contains_name(&variable_name) {
            // TODO: return synthesis error: "assignment missing" here
            match self.get(&variable_name).unwrap() {
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
        scope: String,
        expression: BooleanExpression,
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

    fn get_u32_value<F: Field + PrimeField + PrimeField, CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        scope: String,
        expression: FieldExpression,
    ) -> UInt32 {
        match expression {
            FieldExpression::Variable(variable) => self.u32_from_variable(cs, scope, variable),
            FieldExpression::Number(number) => UInt32::constant(number),
            field => match self.enforce_field_expression(cs, scope, field) {
                ResolvedValue::FieldElement(value) => value,
                _ => unimplemented!("field expression did not resolve to field"),
            },
        }
    }

    fn enforce_not<F: Field + PrimeField, CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        scope: String,
        expression: BooleanExpression,
    ) -> Boolean {
        let expression = self.get_bool_value(cs, scope, expression);

        expression.not()
    }

    fn enforce_or<F: Field + PrimeField, CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        scope: String,
        left: BooleanExpression,
        right: BooleanExpression,
    ) -> Boolean {
        let left = self.get_bool_value(cs, scope.clone(), left);
        let right = self.get_bool_value(cs, scope.clone(), right);

        Boolean::or(cs, &left, &right).unwrap()
    }

    fn enforce_and<F: Field + PrimeField, CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        scope: String,
        left: BooleanExpression,
        right: BooleanExpression,
    ) -> Boolean {
        let left = self.get_bool_value(cs, scope.clone(), left);
        let right = self.get_bool_value(cs, scope.clone(), right);

        Boolean::and(cs, &left, &right).unwrap()
    }

    fn enforce_bool_equality<F: Field + PrimeField, CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        scope: String,
        left: BooleanExpression,
        right: BooleanExpression,
    ) -> Boolean {
        let left = self.get_bool_value(cs, scope.clone(), left);
        let right = self.get_bool_value(cs, scope.clone(), right);

        left.enforce_equal(cs.ns(|| format!("enforce bool equal")), &right)
            .unwrap();

        Boolean::Constant(true)
    }

    fn enforce_field_equality<F: Field + PrimeField, CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        scope: String,
        left: FieldExpression,
        right: FieldExpression,
    ) -> Boolean {
        let left = self.get_u32_value(cs, scope.clone(), left);
        let right = self.get_u32_value(cs, scope.clone(), right);

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
        scope: String,
        expression: BooleanExpression,
    ) -> ResolvedValue {
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
                ResolvedValue::Boolean(self.enforce_field_equality(cs, scope, *left, *right))
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
            BooleanExpression::Array(array) => ResolvedValue::BooleanArray(
                array
                    .into_iter()
                    .map(|element| match *element {
                        BooleanSpreadOrExpression::Spread(_spread) => {
                            unimplemented!("spreads not enforced yet")
                        }
                        BooleanSpreadOrExpression::BooleanExpression(expression) => {
                            match self.enforce_boolean_expression(cs, scope.clone(), expression) {
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
        scope: String,
        left: FieldExpression,
        right: FieldExpression,
    ) -> UInt32 {
        let left = self.get_u32_value(cs, scope.clone(), left);
        let right = self.get_u32_value(cs, scope.clone(), right);

        UInt32::addmany(
            cs.ns(|| format!("enforce {} + {}", left.value.unwrap(), right.value.unwrap())),
            &[left, right],
        )
        .unwrap()
    }

    fn enforce_sub<F: Field + PrimeField, CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        scope: String,
        left: FieldExpression,
        right: FieldExpression,
    ) -> UInt32 {
        let left = self.get_u32_value(cs, scope.clone(), left);
        let right = self.get_u32_value(cs, scope.clone(), right);

        left.sub(
            cs.ns(|| format!("enforce {} - {}", left.value.unwrap(), right.value.unwrap())),
            &right,
        )
        .unwrap()
    }

    fn enforce_mul<F: Field + PrimeField, CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        scope: String,
        left: FieldExpression,
        right: FieldExpression,
    ) -> UInt32 {
        let left = self.get_u32_value(cs, scope.clone(), left);
        let right = self.get_u32_value(cs, scope.clone(), right);

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
        scope: String,
        left: FieldExpression,
        right: FieldExpression,
    ) -> UInt32 {
        let left = self.get_u32_value(cs, scope.clone(), left);
        let right = self.get_u32_value(cs, scope.clone(), right);

        left.div(
            cs.ns(|| format!("enforce {} / {}", left.value.unwrap(), right.value.unwrap())),
            &right,
        )
        .unwrap()
    }

    fn enforce_pow<F: Field + PrimeField, CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        scope: String,
        left: FieldExpression,
        right: FieldExpression,
    ) -> UInt32 {
        let left = self.get_u32_value(cs, scope.clone(), left);
        let right = self.get_u32_value(cs, scope.clone(), right);

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
        scope: String,
        expression: FieldExpression,
    ) -> ResolvedValue {
        match expression {
            FieldExpression::Variable(variable) => {
                ResolvedValue::FieldElement(self.u32_from_variable(cs, scope, variable))
            }
            FieldExpression::Number(number) => {
                ResolvedValue::FieldElement(UInt32::constant(number))
            }
            FieldExpression::Add(left, right) => {
                ResolvedValue::FieldElement(self.enforce_add(cs, scope, *left, *right))
            }
            FieldExpression::Sub(left, right) => {
                ResolvedValue::FieldElement(self.enforce_sub(cs, scope, *left, *right))
            }
            FieldExpression::Mul(left, right) => {
                ResolvedValue::FieldElement(self.enforce_mul(cs, scope, *left, *right))
            }
            FieldExpression::Div(left, right) => {
                ResolvedValue::FieldElement(self.enforce_div(cs, scope, *left, *right))
            }
            FieldExpression::Pow(left, right) => {
                ResolvedValue::FieldElement(self.enforce_pow(cs, scope, *left, *right))
            }
            FieldExpression::IfElse(first, second, third) => {
                let resolved_first =
                    match self.enforce_boolean_expression(cs, scope.clone(), *first) {
                        ResolvedValue::Boolean(resolved) => resolved,
                        _ => unimplemented!("if else conditional must resolve to boolean"),
                    };

                if resolved_first.eq(&Boolean::Constant(true)) {
                    self.enforce_field_expression(cs, scope, *second)
                } else {
                    self.enforce_field_expression(cs, scope, *third)
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
                            match self.enforce_field_expression(cs, scope.clone(), expression) {
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
        scope: String,
        variable: Variable,
        members: Vec<StructMember>,
    ) -> ResolvedValue {
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

    fn enforce_index<F: Field + PrimeField, CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        scope: String,
        index: FieldExpression,
    ) -> usize {
        match self.enforce_field_expression(cs, scope.clone(), index) {
            ResolvedValue::FieldElement(number) => number.value.unwrap() as usize,
            value => unimplemented!("From index must resolve to a uint32, got {}", value),
        }
    }

    fn enforce_array_access_expression<F: Field + PrimeField, CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        scope: String,
        array: Box<Expression>,
        index: FieldRangeOrExpression,
    ) -> ResolvedValue {
        match self.enforce_expression(cs, scope.clone(), *array) {
            ResolvedValue::FieldElementArray(field_array) => {
                match index {
                    FieldRangeOrExpression::Range(from, to) => {
                        let from_resolved = match from {
                            Some(from_index) => self.enforce_index(cs, scope.clone(), from_index),
                            None => 0usize, // Array slice starts at index 0
                        };
                        let to_resolved = match to {
                            Some(to_index) => self.enforce_index(cs, scope.clone(), to_index),
                            None => field_array.len(), // Array slice ends at array length
                        };
                        ResolvedValue::FieldElementArray(
                            field_array[from_resolved..to_resolved].to_owned(),
                        )
                    }
                    FieldRangeOrExpression::FieldExpression(index) => {
                        let index_resolved = self.enforce_index(cs, scope.clone(), index);
                        ResolvedValue::FieldElement(field_array[index_resolved].to_owned())
                    }
                }
            }
            ResolvedValue::BooleanArray(bool_array) => {
                match index {
                    FieldRangeOrExpression::Range(from, to) => {
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
                    FieldRangeOrExpression::FieldExpression(index) => {
                        let index_resolved = self.enforce_index(cs, scope.clone(), index);
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
        scope: String,
        struct_variable: Box<Expression>,
        struct_member: Variable,
    ) -> ResolvedValue {
        match self.enforce_expression(cs, scope.clone(), *struct_variable) {
            ResolvedValue::StructExpression(_name, members) => {
                let matched_member = members
                    .into_iter()
                    .find(|member| member.variable == struct_member);
                match matched_member {
                    Some(member) => self.enforce_expression(cs, scope.clone(), member.expression),
                    None => unimplemented!("Cannot access struct member {}", struct_member.0),
                }
            }
            value => unimplemented!("Cannot access element of untyped struct {}", value),
        }
    }

    fn enforce_function_access_expression<F: Field + PrimeField, CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        scope: String,
        function: Box<Expression>,
        arguments: Vec<Expression>,
    ) -> ResolvedValue {
        match self.enforce_expression(cs, scope, *function) {
            ResolvedValue::Function(function) => self.enforce_function(cs, function, arguments),
            value => unimplemented!("Cannot call unknown function {}", value),
        }
    }

    fn enforce_expression<F: Field + PrimeField, CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        scope: String,
        expression: Expression,
    ) -> ResolvedValue {
        match expression {
            Expression::Boolean(boolean_expression) => {
                self.enforce_boolean_expression(cs, scope, boolean_expression)
            }
            Expression::FieldElement(field_expression) => {
                self.enforce_field_expression(cs, scope, field_expression)
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
                        ResolvedValue::FieldElement(self.u32_from_variable(
                            cs,
                            variable_name,
                            unresolved_variable,
                        ))
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
        }
    }

    fn resolve_assignee(&mut self, scope: String, assignee: Assignee) -> String {
        match assignee {
            Assignee::Variable(name) => new_scope_from_variable(scope, &name),
            Assignee::Array(array, _index) => self.resolve_assignee(scope, *array),
            Assignee::StructMember(struct_variable, _member) => {
                self.resolve_assignee(scope, *struct_variable)
            }
        }
    }

    // fn modify_array<F: Field + PrimeField, CS: ConstraintSystem<F>>(
    //     &mut self,
    //     cs: &mut CS,
    //     scope: String,
    //     assignee: Assignee,
    //     expression: Expression,
    // )

    fn enforce_definition_statement<F: Field + PrimeField, CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        scope: String,
        assignee: Assignee,
        expression: Expression,
    ) {
        // Create or modify the lhs variable in the current function scope
        match assignee {
            Assignee::Variable(name) => {
                // Store the variable in the current scope
                let definition_name = new_scope_from_variable(scope.clone(), &name);
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
                    FieldRangeOrExpression::FieldExpression(index) => {
                        let index = self.enforce_index(cs, scope.clone(), index);

                        // Modify the single value of the array in place
                        match self.get_mut(&expected_array_name) {
                            Some(value) => match (value, result) {
                                (
                                    ResolvedValue::FieldElementArray(old),
                                    ResolvedValue::FieldElement(new),
                                ) => {
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
                    FieldRangeOrExpression::Range(from, to) => {
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
                                (
                                    ResolvedValue::FieldElementArray(old),
                                    ResolvedValue::FieldElementArray(new),
                                ) => {
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

    fn enforce_return_statement<F: Field + PrimeField, CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        scope: String,
        statements: Vec<Expression>,
    ) -> ResolvedValue {
        ResolvedValue::Return(
            statements
                .into_iter()
                .map(|expression| self.enforce_expression(cs, scope.clone(), expression))
                .collect::<Vec<ResolvedValue>>(),
        )
    }

    fn enforce_statement<F: Field + PrimeField, CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        scope: String,
        statement: Statement,
    ) {
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

    fn enforce_for_statement<F: Field + PrimeField, CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        scope: String,
        index: Variable,
        start: FieldExpression,
        stop: FieldExpression,
        statements: Vec<Statement>,
    ) {
        let start_index = self.enforce_index(cs, scope.clone(), start);
        let stop_index = self.enforce_index(cs, scope.clone(), stop);

        for i in start_index..stop_index {
            // Store index in current function scope.
            // For loop scope is not implemented.
            let index_name = new_scope_from_variable(scope.clone(), &index);
            self.store(
                index_name,
                ResolvedValue::FieldElement(UInt32::constant(i as u32)),
            );

            // Evaluate statements
            statements
                .clone()
                .into_iter()
                .for_each(|statement| self.enforce_statement(cs, scope.clone(), statement));
        }
    }

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
                        match self.enforce_expression(cs, function.name(), argument) {
                            ResolvedValue::FieldElement(field) => {
                                // Store argument as variable with {function_name}_{parameter name}
                                let variable_name =
                                    new_scope_from_variable(function.name(), &parameter.variable);
                                self.store(variable_name, ResolvedValue::FieldElement(field));
                            }
                            argument => unimplemented!("expected field argument got {}", argument),
                        }
                    }
                    Type::Boolean => match self.enforce_expression(cs, function.name(), argument) {
                        ResolvedValue::Boolean(bool) => {
                            // Store argument as variable with {function_name}_{parameter name}
                            let variable_name =
                                new_scope_from_variable(function.name(), &parameter.variable);
                            self.store(variable_name, ResolvedValue::Boolean(bool));
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
                    self.enforce_definition_statement(cs, function.name(), variable, expression);
                }
                Statement::For(index, start, stop, statements) => {
                    self.enforce_for_statement(cs, function.name(), index, start, stop, statements);
                }
                Statement::Return(expressions) => {
                    return_values = self.enforce_return_statement(cs, function.name(), expressions)
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
                    .store_variable(variable, ResolvedValue::StructDefinition(struct_def));
            });
        program
            .functions
            .into_iter()
            .for_each(|(function_name, function)| {
                resolved_program.store(function_name.0, ResolvedValue::Function(function));
            });

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
