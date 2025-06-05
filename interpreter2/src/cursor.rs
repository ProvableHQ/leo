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

use leo_ast::{
    Block,
    Expression,
    Location,
    Statement,
    interpreter_value::{Address, Future, LeoValue, Plaintext, PlaintextHash, Value},
};
use leo_errors::Result;
use leo_span::Symbol;

use indexmap::IndexMap;
use rand_chacha::{ChaCha20Rng, rand_core::SeedableRng as _};

#[derive(Clone, Debug)]
pub enum BlockOrStatement<'a> {
    Block(&'a Block),
    Statement(&'a Statement),
}

impl<'a> From<&'a Block> for BlockOrStatement<'a> {
    fn from(value: &'a Block) -> Self {
        BlockOrStatement::Block(value)
    }
}

impl<'a> From<&'a Statement> for BlockOrStatement<'a> {
    fn from(value: &'a Statement) -> Self {
        BlockOrStatement::Statement(value)
    }
}

#[derive(Clone, Debug)]
pub struct StatementFrame<'a> {
    pub step: usize,
    pub statement: BlockOrStatement<'a>,
    pub expression_frames: Vec<ExpressionFrame<'a>>,
    // Stack of evaluated values.
    pub values: Vec<LeoValue>,
}

impl<'a> From<&'a Block> for StatementFrame<'a> {
    fn from(value: &'a Block) -> Self {
        Self { step: 0, statement: value.into(), expression_frames: Default::default(), values: Default::default() }
    }
}

impl<'a> From<&'a Statement> for StatementFrame<'a> {
    fn from(value: &'a Statement) -> Self {
        Self { step: 0, statement: value.into(), expression_frames: Default::default(), values: Default::default() }
    }
}

#[derive(Clone, Debug)]
pub struct ExpressionFrame<'a> {
    pub step: usize,
    pub expression: &'a Expression,
}

impl<'a> From<&'a Expression> for ExpressionFrame<'a> {
    fn from(value: &'a Expression) -> Self {
        Self { step: 0, expression: value }
    }
}

#[derive(Clone, Debug)]
pub struct AleoFunctionCall {
    registers: Vec<LeoValue>,
}

/// Names associated to values in a function being executed.
#[derive(Clone, Debug)]
pub struct LeoFunctionCall<'a> {
    pub program: Symbol,
    pub caller: Address,
    pub names: IndexMap<Symbol, LeoValue>,
    pub accumulated_futures: Vec<Future>,
    pub is_async: bool,
    pub statement_frames: Vec<StatementFrame<'a>>,
}

#[derive(Debug)]
enum ValueOrPlaintext<'a> {
    Value(&'a mut Value),
    LeoValue(&'a mut LeoValue),
    Plaintext(&'a mut Plaintext),
}

impl LeoFunctionCall<'_> {
    fn set_place(
        new_value: LeoValue,
        this_value: ValueOrPlaintext<'_>,
        places: &mut dyn Iterator<Item = &Expression>,
        indices: &mut dyn Iterator<Item = LeoValue>,
    ) -> Result<()> {
        match places.next() {
            None => match this_value {
                ValueOrPlaintext::Value(value) => {
                    let svm_value = new_value.try_into().expect("NO");
                    *value = svm_value;
                    Ok(())
                }
                ValueOrPlaintext::LeoValue(leo_value) => {
                    *leo_value = new_value;
                    Ok(())
                }
                ValueOrPlaintext::Plaintext(plaintext) => {
                    let plaintext_value = new_value.try_into().expect("NO");
                    *plaintext = plaintext_value;
                    Ok(())
                }
            },
            Some(Expression::ArrayAccess(_access)) => {
                let index = indices.next().expect("NO");
                let index = index.try_as_usize().expect("NO");
                let array = match this_value {
                    ValueOrPlaintext::LeoValue(leo_value) => leo_value.try_as_array_mut().expect("NO"),
                    ValueOrPlaintext::Plaintext(plaintext)
                    | ValueOrPlaintext::Value(snarkvm::prelude::Value::Plaintext(plaintext)) => match plaintext {
                        snarkvm::prelude::Plaintext::Array(vec, _) => vec,
                        _ => panic!("NO"),
                    },
                    _ => panic!("NO"),
                };
                Self::set_place(new_value, ValueOrPlaintext::Plaintext(&mut array[index]), places, indices)
            }
            Some(Expression::TupleAccess(access)) => {
                let index = access.index.value();
                let ValueOrPlaintext::LeoValue(LeoValue::Tuple(vec)) = this_value else {
                    panic!("NO");
                };
                Self::set_place(new_value, ValueOrPlaintext::Value(&mut vec[index]), places, indices)
            }
            Some(Expression::MemberAccess(access)) => {
                let mut value: LeoValue = match &this_value {
                    ValueOrPlaintext::Value(value) => (**value).clone().into(),
                    ValueOrPlaintext::LeoValue(leo_value) => (**leo_value).clone(),
                    ValueOrPlaintext::Plaintext(plaintext) => (**plaintext).clone().into(),
                };

                let mut new_this_value = value.member_get(access.name.name).expect("NO");

                Self::set_place(new_value, ValueOrPlaintext::Plaintext(&mut new_this_value), places, indices)?;

                value.member_set(access.name.name, new_this_value.into());

                match this_value {
                    ValueOrPlaintext::Value(svm_value) => *svm_value = value.try_into().expect("NO"),
                    ValueOrPlaintext::LeoValue(leo_value) => *leo_value = value,
                    ValueOrPlaintext::Plaintext(plaintext) => *plaintext = value.try_into().expect("NO"),
                }

                Ok(())
            }
            Some(..) => panic!("NO"),
        }
    }

    pub fn assign(
        &mut self,
        value: LeoValue,
        place: &Expression,
        indices: &mut dyn Iterator<Item = LeoValue>,
    ) -> Result<()> {
        let mut places = vec![place];

        loop {
            match places.last().unwrap() {
                Expression::ArrayAccess(access) => places.push(&access.array),
                Expression::TupleAccess(access) => places.push(&access.tuple),
                Expression::MemberAccess(access) => places.push(&access.inner),
                Expression::Identifier(..) => break,
                Expression::AssociatedConstant(..)
                | Expression::AssociatedFunction(..)
                | Expression::Array(..)
                | Expression::Binary(..)
                | Expression::Call(..)
                | Expression::Cast(..)
                | Expression::Err(..)
                | Expression::Literal(..)
                | Expression::Locator(..)
                | Expression::Struct(..)
                | Expression::Ternary(..)
                | Expression::Tuple(..)
                | Expression::Unary(..)
                | Expression::Unit(..) => panic!("NO"),
            }
        }

        let Some(Expression::Identifier(id)) = places.pop() else {
            panic!("NO");
        };

        let leo_value = self.names.entry(id.name).or_insert(Default::default());

        Self::set_place(value, ValueOrPlaintext::LeoValue(leo_value), &mut places.into_iter().rev(), indices)
    }
}

#[derive(Clone, Debug)]
pub enum FunctionCall<'a> {
    Aleo(AleoFunctionCall),
    Leo(LeoFunctionCall<'a>),
}

impl<'a> From<AleoFunctionCall> for FunctionCall<'a> {
    fn from(value: AleoFunctionCall) -> Self {
        FunctionCall::Aleo(value)
    }
}

impl<'a> From<LeoFunctionCall<'a>> for FunctionCall<'a> {
    fn from(value: LeoFunctionCall<'a>) -> Self {
        FunctionCall::Leo(value)
    }
}

#[derive(Clone, Debug)]
pub struct Cursor<'a> {
    pub mappings: IndexMap<Location, IndexMap<PlaintextHash, LeoValue>>,

    pub globals: IndexMap<Location, LeoValue>,

    pub function_call_stack: Vec<FunctionCall<'a>>,

    pub last_return_value: Option<LeoValue>,

    pub rng: ChaCha20Rng,

    pub signer: Address,

    pub block_height: u32,
}

impl Cursor<'_> {
    pub fn new(
        mappings: impl IntoIterator<Item = Location>,
        globals: impl IntoIterator<Item = (Location, LeoValue)>,
        block_height: u32,
        signer: Address,
    ) -> Self {
        Self {
            mappings: mappings.into_iter().map(|location| (location, Default::default())).collect(),
            globals: globals.into_iter().collect(),
            function_call_stack: Default::default(),
            last_return_value: None,
            rng: ChaCha20Rng::seed_from_u64(1),
            signer,
            block_height,
        }
    }
}
