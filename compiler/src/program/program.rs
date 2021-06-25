// Copyright (C) 2019-2021 Aleo Systems Inc.
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

//! An in memory store to keep track of defined names when constraining a Leo program.

use leo_asg::{CircuitMember, Function, FunctionQualifier, InputCategory, IntegerType, Type as AsgType, Variable};
use snarkvm_ir::{Header, Instruction, MaskData, QueryData, RepeatData, SnarkVMVersion, Type, Value};

use indexmap::IndexMap;

#[derive(Clone, Debug)]
struct Input {
    name: String,
    category: InputCategory,
    type_: Type,
}

#[derive(Clone, Debug)]
pub(crate) struct IrFunction {
    argument_start_variable: u32,
    instructions: Vec<Instruction>,
}

#[derive(Clone, Debug)]
pub(crate) struct Program<'a> {
    pub asg: leo_asg::Program<'a>,
    pub current_function: Option<&'a Function<'a>>,
    functions: Vec<IrFunction>,
    function_to_index: IndexMap<u32, u32>,
    next_register: u32,
    inputs: IndexMap<u32, Input>,
    variable_to_register: IndexMap<u32, u32>,
    input_orderings: IndexMap<InputCategory, Vec<String>>,
}

impl<'a> Program<'a> {
    pub fn new(asg: leo_asg::Program<'a>) -> Self {
        Self {
            asg,
            current_function: None,
            functions: vec![],
            function_to_index: IndexMap::new(),
            next_register: 0,
            inputs: IndexMap::new(),
            variable_to_register: IndexMap::new(),
            input_orderings: IndexMap::new(),
        }
    }

    pub fn alloc(&mut self) -> u32 {
        let register = self.next_register;
        self.next_register += 1;
        register
    }

    pub fn alloc_var(&mut self, variable: &Variable<'a>) -> u32 {
        let register = self.alloc();
        if self
            .variable_to_register
            .insert(variable.borrow().id, register)
            .is_some()
        {
            panic!("illegal reallocation of variable {}", variable.borrow().name);
        }
        register
    }

    pub fn alloc_input(&mut self, category: InputCategory, name: &str, type_: AsgType<'a>) -> u32 {
        let variable = self.alloc();
        self.inputs.insert(variable, Input {
            category,
            type_: asg_to_ir_type(&type_),
            name: name.to_string(),
        });
        variable
    }

    pub fn alloc_input_var(&mut self, category: InputCategory, variable: &Variable<'a>) -> u32 {
        let type_ = variable.borrow().type_.clone();
        let register = self.alloc();
        self.variable_to_register.insert(variable.borrow().id, register);
        self.inputs.insert(register, Input {
            category,
            type_: asg_to_ir_type(&type_),
            name: variable.borrow().name.name.to_string(),
        });
        register
    }

    pub(crate) fn register_section_names(&mut self, category: InputCategory, names: Vec<String>) {
        self.input_orderings.insert(category, names);
    }

    pub fn input_index(&mut self, category: InputCategory, name: &str) -> usize {
        self.input_orderings
            .get(&category)
            .map(|x| x.iter().position(|x| &**x == name).expect("missing input for index"))
            .expect("missing input category for index")
    }

    pub fn resolve_var(&self, variable: &Variable<'a>) -> u32 {
        *self
            .variable_to_register
            .get(&variable.borrow().id)
            .expect("missing register allocation for variable")
    }

    /// returns function index
    pub fn resolve_function(&self, function: &leo_asg::Function<'a>) -> u32 {
        *self.function_to_index.get(&function.id).expect("unresolved function")
    }

    pub fn register_function(&mut self, function: &'a Function<'a>) {
        let len = self.function_to_index.len();
        self.function_to_index.insert(function.id, len as u32);
    }

    pub fn begin_main_function(&mut self, function: &'a Function<'a>) {
        self.current_function = Some(function);
        self.functions.push(IrFunction {
            instructions: vec![],
            argument_start_variable: self.next_register,
        });
    }

    pub fn begin_function(&mut self, function: &'a Function<'a>) {
        self.current_function = Some(function);
        // let resolved = self.resolve_function(function);
        self.functions.push(IrFunction {
            instructions: vec![],
            argument_start_variable: self.next_register,
        });
        match function.qualifier {
            FunctionQualifier::Static => (),
            _ => {
                let self_var = function
                    .scope
                    .resolve_variable("self")
                    .expect("missing self var for function");
                self.alloc_var(self_var); // alloc space for self representation
            }
        }
        for (_name, variable) in &function.arguments {
            self.alloc_var(variable.get());
        }
    }

    fn current_instructions(&mut self) -> &mut Vec<Instruction> {
        &mut self.functions.last_mut().unwrap().instructions
    }

    pub fn emit(&mut self, instruction: Instruction) {
        self.current_instructions().push(instruction);
    }

    pub fn store(&mut self, target: &Variable<'a>, value: Value) {
        self.emit(Instruction::Store(QueryData {
            destination: self.resolve_var(target),
            values: vec![value],
        }));
    }

    pub fn mask<R, E, F: FnMut(&mut Self) -> Result<R, E>>(&mut self, condition: Value, mut inner: F) -> Result<R, E> {
        let start_index = self.current_instructions().len();
        let out = inner(self);
        if out.is_err() {
            self.current_instructions().truncate(start_index);
            return out;
        }
        let masked_count = self.current_instructions().len() - start_index;
        self.current_instructions().insert(
            start_index,
            Instruction::Mask(MaskData {
                instruction_count: masked_count as u32,
                condition,
            }),
        );
        out
    }

    pub fn repeat<R, E, F: FnMut(&mut Self) -> Result<R, E>>(
        &mut self,
        iter_variable: &Variable<'a>,
        from: Value,
        to: Value,
        mut inner: F,
    ) -> Result<R, E> {
        let start_index = self.current_instructions().len();
        let iter_register = self.alloc_var(iter_variable);
        let out = inner(self);
        if out.is_err() {
            self.current_instructions().truncate(start_index);
            return out;
        }
        let masked_count = self.current_instructions().len() - start_index;
        self.current_instructions().insert(
            start_index,
            Instruction::Repeat(RepeatData {
                instruction_count: masked_count as u32,
                iter_variable: iter_register,
                from,
                to,
            }),
        );
        out
    }

    fn prepare_inputs(&self, category: InputCategory) -> Vec<snarkvm_ir::Input> {
        self.inputs
            .iter()
            .filter(|(_, Input { category: cat, .. })| cat == &category)
            .map(|(register, input)| snarkvm_ir::Input {
                variable: *register,
                name: input.name.clone(),
                type_: input.type_.clone(),
            })
            .collect()
    }

    pub fn render(self) -> snarkvm_ir::Program {
        let main_inputs = self.prepare_inputs(InputCategory::MainInput);
        let constant_inputs = self.prepare_inputs(InputCategory::ConstInput);
        let register_inputs = self.prepare_inputs(InputCategory::Register);
        let public_states = self.prepare_inputs(InputCategory::PublicState);
        let private_record_states = self.prepare_inputs(InputCategory::StateRecord);
        let private_leaf_states = self.prepare_inputs(InputCategory::StateLeaf);

        snarkvm_ir::Program {
            functions: self
                .functions
                .into_iter()
                .map(|f| snarkvm_ir::Function {
                    argument_start_variable: f.argument_start_variable,
                    instructions: f.instructions,
                })
                .collect(),
            header: Header {
                version: SnarkVMVersion::default(),
                main_inputs,
                constant_inputs,
                register_inputs,
                public_states,
                private_record_states,
                private_leaf_states,
            },
        }
    }
}


fn asg_to_ir_type(type_: &AsgType) -> Type {
    match type_ {
        AsgType::Address => Type::Address,
        AsgType::Boolean => Type::Boolean,
        AsgType::Char => Type::Char,
        AsgType::Field => Type::Field,
        AsgType::Group => Type::Group,
        AsgType::Integer(int_type) => match int_type {
            IntegerType::U8 => Type::U8,
            IntegerType::U16 => Type::U16,
            IntegerType::U32 => Type::U32,
            IntegerType::U64 => Type::U64,
            IntegerType::U128 => Type::U128,
            IntegerType::I8 => Type::I8,
            IntegerType::I16 => Type::I16,
            IntegerType::I32 => Type::I32,
            IntegerType::I64 => Type::I64,
            IntegerType::I128 => Type::I128,
        },
        AsgType::Array(inner, len) => Type::Array(Box::new(asg_to_ir_type(&*inner)), *len),
        AsgType::Tuple(items) => Type::Tuple(items.iter().map(asg_to_ir_type).collect()),
        AsgType::Circuit(circuit) => {
            let members = circuit.members.borrow();
            let members = members
                .iter()
                .flat_map(|(_, member)| match member {
                    CircuitMember::Variable(type_) => Some(asg_to_ir_type(type_)),
                    CircuitMember::Function(_) => None,
                })
                .collect();
            Type::Tuple(members)
        }
    }
}
