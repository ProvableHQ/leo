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

use crate::CompilerState;

use leo_ast::{
    ArrayExpression,
    Expression,
    Identifier,
    IntegerType,
    NodeID,
    StructExpression,
    StructVariableInitializer,
    Type,
    interpreter_value::{Address, LeoValue, Plaintext},
};
use leo_errors::StaticAnalyzerError;
use leo_span::{Span, Symbol, sym};

use std::{fmt::Write as _, str::FromStr};

pub struct ConstPropagationVisitor<'a> {
    pub state: &'a mut CompilerState,
    /// The program name.
    pub program: Symbol,
    /// Have we actually modified the program at all?
    pub changed: bool,
    /// The RHS of a const declaration we were not able to evaluate.
    pub const_not_evaluated: Option<Span>,
    /// An array index which was not able to be evaluated.
    pub array_index_not_evaluated: Option<Span>,

    pub propagate_through_let: bool,
}

impl ConstPropagationVisitor<'_> {
    /// Enter the symbol table's scope `id`, execute `func`, and then return to the parent scope.
    pub fn in_scope<T>(&mut self, id: NodeID, func: impl FnOnce(&mut Self) -> T) -> T {
        self.state.symbol_table.enter_scope(Some(id));
        let result = func(self);
        self.state.symbol_table.enter_parent();
        result
    }

    /// Emit a `StaticAnalyzerError`.
    pub fn emit_err(&self, err: StaticAnalyzerError) {
        self.state.handler.emit_err(err);
    }

    fn plaintext_to_expression(&self, plaintext: &Plaintext, span: Span, ty: &Type) -> Option<Expression> {
        use leo_ast::Literal as LeoLiteral;

        let id = self.state.node_builder.next_id();

        let result: Expression = match plaintext {
            Plaintext::Literal(literal, _) => {
                use snarkvm::prelude::Literal::*;
                match literal {
                    Address(x) => LeoLiteral::address(x.to_string(), span, id).into(),
                    Boolean(x) => LeoLiteral::boolean(**x, span, id).into(),
                    Field(x) => {
                        let mut s = x.to_string();
                        // Strip off the `field` suffix.
                        s.truncate(s.len() - 5);
                        LeoLiteral::field(s, span, id).into()
                    }
                    Group(x) => {
                        let mut s = x.to_string();
                        // Strip off the `group` suffix.
                        s.truncate(s.len() - 5);
                        LeoLiteral::group(s, span, id).into()
                    }
                    // For the integers, we make sure to apply `to_string` to the native Rust integer, not the snarkVM
                    // integer type, so that we don't get the type suffix.
                    I8(x) => LeoLiteral::integer(IntegerType::I8, (**x).to_string(), span, id).into(),
                    I16(x) => LeoLiteral::integer(IntegerType::I16, (**x).to_string(), span, id).into(),
                    I32(x) => LeoLiteral::integer(IntegerType::I32, (**x).to_string(), span, id).into(),
                    I64(x) => LeoLiteral::integer(IntegerType::I64, (**x).to_string(), span, id).into(),
                    I128(x) => LeoLiteral::integer(IntegerType::I128, (**x).to_string(), span, id).into(),
                    U8(x) => LeoLiteral::integer(IntegerType::U8, (**x).to_string(), span, id).into(),
                    U16(x) => LeoLiteral::integer(IntegerType::U16, (**x).to_string(), span, id).into(),
                    U32(x) => LeoLiteral::integer(IntegerType::U32, (**x).to_string(), span, id).into(),
                    U64(x) => LeoLiteral::integer(IntegerType::U64, (**x).to_string(), span, id).into(),
                    U128(x) => LeoLiteral::integer(IntegerType::U128, (**x).to_string(), span, id).into(),
                    Scalar(x) => {
                        let mut s = x.to_string();
                        // Strip off the `scalar` suffix.
                        s.truncate(s.len() - 6);
                        LeoLiteral::scalar(s, span, id).into()
                    }
                    Signature(_signature) => todo!(),
                    String(_string_type) => todo!(),
                }
            }
            Plaintext::Struct(index_map, _) => {
                let Type::Composite(comp_ty) = ty else {
                    panic!("Can't happen");
                };
                let composite = self.state.symbol_table.lookup_struct(comp_ty.id.name).expect("");

                let mut members = Vec::with_capacity(composite.members.len());
                let mut buffer = String::new();
                for member in &composite.members {
                    buffer.clear();
                    write!(&mut buffer, "{}", member.identifier).unwrap();
                    let value = index_map
                        .get(&snarkvm::prelude::Identifier::from_str(&buffer).unwrap())
                        .expect("Struct should have all fields");
                    let member_expr = self.plaintext_to_expression(value, span, &member.type_)?;
                    members.push(StructVariableInitializer {
                        identifier: member.identifier,
                        expression: Some(member_expr),
                        span,
                        id: self.state.node_builder.next_id(),
                    });
                }

                StructExpression { name: comp_ty.id, members, span, id }.into()
            }
            Plaintext::Array(x, _) => {
                let mut elements = Vec::with_capacity(x.len());
                for value in x.iter() {
                    elements.push(self.plaintext_to_expression(value, span, ty)?);
                }
                ArrayExpression { elements, span, id }.into()
            }
        };

        Some(result)
    }

    pub fn value_to_expression(&self, value: &LeoValue, span: Span, ty: &Type) -> Option<Expression> {
        use LeoValue::*;
        use snarkvm::prelude::Value as SvmValue;

        let result = match value {
            Unit => leo_ast::UnitExpression { span, id: self.state.node_builder.next_id() }.into(),
            Tuple(tuple) => {
                let Type::Tuple(tuple_ty) = ty else {
                    panic!("Can't happen");
                };
                assert!(tuple_ty.elements().len() == tuple.len());
                let mut elements = Vec::with_capacity(tuple.len());
                for (value, value_ty) in tuple.iter().zip(tuple_ty.elements()) {
                    elements.push(self.value_to_expression(&value.clone().into(), span, value_ty)?);
                }
                leo_ast::TupleExpression { elements, span, id: self.state.node_builder.next_id() }.into()
            }
            Value(SvmValue::Plaintext(plaintext)) => self.plaintext_to_expression(plaintext, span, ty)?,
            Value(SvmValue::Record(record)) => {
                let mut buffer = String::new();
                let Type::Composite(comp_ty) = ty else {
                    panic!("Can't happen");
                };
                let program = comp_ty.program.unwrap_or(self.program);
                let location = leo_ast::Location::new(program, comp_ty.id.name);
                let composite = self.state.symbol_table.lookup_record(location).expect("");

                let owner_address: &Address = &*record.owner();
                let owner_member = StructVariableInitializer {
                    identifier: Identifier::new(sym::owner, self.state.node_builder.next_id()),
                    expression: Some(
                        leo_ast::Literal::address(owner_address.to_string(), span, self.state.node_builder.next_id())
                            .into(),
                    ),
                    span,
                    id: self.state.node_builder.next_id(),
                };

                let mut members = vec![owner_member];

                // Add other members from the record
                let data = record.data();
                for member in &composite.members {
                    if member.identifier.name != sym::owner {
                        buffer.clear();
                        write!(&mut buffer, "{}", member.identifier).unwrap();
                        let entry = data
                            .get(&snarkvm::prelude::Identifier::from_str(&buffer).unwrap())
                            .expect("Record should have all fields");
                        let plaintext = match entry {
                            snarkvm::prelude::Entry::Constant(plaintext)
                            | snarkvm::prelude::Entry::Public(plaintext) => plaintext,
                            snarkvm::prelude::Entry::Private(_) => return None,
                        };
                        let member_expr = self.plaintext_to_expression(plaintext, span, &member.type_)?;
                        members.push(StructVariableInitializer {
                            identifier: member.identifier,
                            expression: Some(member_expr),
                            span,
                            id: self.state.node_builder.next_id(),
                        });
                    }
                }

                StructExpression {
                    name: Identifier::new(comp_ty.id.name, self.state.node_builder.next_id()),
                    members,
                    span,
                    id: self.state.node_builder.next_id(),
                }
                .into()
            }
            Value(SvmValue::Future(..)) => return None,
        };

        Some(result)
    }
}
