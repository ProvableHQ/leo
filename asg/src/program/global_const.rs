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

use crate::{
    AsgConvertError,
    Expression,
    ExpressionNode,
    FromAst,
    InnerVariable,
    Node,
    Scope,
    Span,
    Statement,
    Variable,
};

use std::cell::{Cell, RefCell};

#[derive(Clone)]
pub struct GlobalConst<'a> {
    pub parent: Cell<Option<&'a Statement<'a>>>,
    pub span: Option<Span>,
    pub variable: &'a Variable<'a>,
    pub value: Cell<&'a Expression<'a>>,
}

// impl<'a> Node for GlobalConst<'a> {
//     fn span(&self) -> Option<&Span> {
//         self.span.as_ref()
//     }
// }

// impl<'a> GlobalConst<'a> {
//     pub(super) fn init(
//         scope: &'a Scope<'a>,
//         global_const: &leo_ast::GlobalConst,
//     ) -> Result<&'a GlobalConst<'a>, AsgConvertError> {
//         let type_ = global_const
//             .type_
//             .as_ref()
//             .map(|x| scope.resolve_ast_type(&x))
//             .transpose()?;

//         let value = <&Expression<'a>>::from_ast(scope, &global_const.value, type_.clone().map(Into::into))?;

//         let type_ = type_.or_else(|| value.get_type());

//         let variable = scope.alloc_variable(RefCell::new(InnerVariable {
//             id: scope.context.get_id(),
//             name: global_const.variable_name.identifier.clone(),
//             type_: type_.ok_or_else(|| {
//                 AsgConvertError::unresolved_type(&global_const.variable_name.identifier.name, &global_const.span)
//             })?,
//             mutable: global_const.variable_name.mutable,
//             const_: false,
//             declaration: crate::VariableDeclaration::Definition,
//             references: vec![],
//             assignments: vec![],
//         }));

//         let global_const = scope.alloc_global_const(GlobalConst {
//             parent: Cell::new(None),
//             span: Some(global_const.span.clone()),
//             variable,
//             value: Cell::new(value),
//         });

//         Ok(global_const)
//     }
// }

// impl<'a> Into<leo_ast::GlobalConst> for &GlobalConst<'a> {
//     fn into(self) -> leo_ast::GlobalConst {
//         let mut type_ = None::<leo_ast::Type>;
//         let variable = self.variable.borrow();
//         let variable_name = leo_ast::VariableName {
//             mutable: variable.mutable,
//             identifier: variable.name.clone(),
//             span: variable.name.span.clone(),
//         };
//         if type_.is_none() {
//             type_ = Some((&variable.type_.clone()).into());
//         }

//         leo_ast::GlobalConst {
//             declaration_type: leo_ast::Declare::Let,
//             variable_name,
//             type_,
//             value: self.value.get().into(),
//             span: self.span.clone().unwrap_or_default(),
//         }
//     }
// }
