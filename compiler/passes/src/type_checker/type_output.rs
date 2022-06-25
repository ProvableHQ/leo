use leo_ast::{ParamMode, Type};

use crate::{Declaration, Value, VariableSymbol};

#[derive(Clone, Debug)]
pub enum TypeOutput {
    // There is a type to return.
    Const(Value),
    ConstType(Type),
    Lit(Value),
    LitType(Type),
    Mut(Value),
    MutType(Type),
    // No information to return
    None,
}

impl TypeOutput {
    pub fn return_incorrect_type(&self, other: &Self, expected: &Option<Type>) -> Self {
        use TypeOutput::*;
        match (self, other) {
            (TypeOutput::None, _) | (_, TypeOutput::None) => TypeOutput::None,
            (ConstType(t1), ConstType(t2)) if t1 == t2 => ConstType(*t1),
            (LitType(t1), LitType(t2))
            | (ConstType(t1), LitType(t2))
            | (MutType(t1), LitType(t2))
            | (LitType(t1), ConstType(t2))
            | (LitType(t1), MutType(t2))
                if t1 == t2 =>
            {
                LitType(*t1)
            }
            (MutType(t1), MutType(t2)) | (ConstType(t1), MutType(t2)) | (MutType(t1), ConstType(t2)) if t1 == t2 => {
                MutType(*t1)
            }
            (Const(t1), Const(t2)) if Type::from(t1) == Type::from(t2) => ConstType(t1.into()),
            (Lit(t1), Lit(t2))
            | (Lit(t1), Const(t2))
            | (Lit(t1), Mut(t2))
            | (Const(t1), Lit(t2))
            | (Mut(t1), Lit(t2))
                if Type::from(t1) == Type::from(t2) =>
            {
                LitType(t1.into())
            }
            (Mut(t1), Mut(t2)) | (Mut(t1), Const(t2)) | (Const(t1), Mut(t2)) if Type::from(t1) == Type::from(t2) => {
                MutType(t1.into())
            }

            (Const(v), ConstType(t)) | (ConstType(t), Const(v)) if t == &Type::from(v) => ConstType(*t),

            (Const(v), LitType(t))
            | (ConstType(t), Lit(v))
            | (Lit(v), ConstType(t))
            | (Lit(v), LitType(t))
            | (Lit(v), MutType(t))
            | (LitType(t), Lit(v))
            | (LitType(t), Const(v))
            | (LitType(t), Mut(v))
            | (Mut(v), LitType(t))
            | (MutType(t), Lit(v))
                if t == &Type::from(v) =>
            {
                LitType(*t)
            }

            (Const(v), MutType(t))
            | (ConstType(t), Mut(v))
            | (Mut(v), ConstType(t))
            | (Mut(v), MutType(t))
            | (MutType(t), Const(v))
            | (MutType(t), Mut(v))
                if t == &Type::from(v) =>
            {
                MutType(*t)
            }
            (left, right) => {
                let t1: Option<Type> = left.into();
                let t2: Option<Type> = right.into();

                if let Some(expected) = expected {
                    if t1.as_ref().unwrap() != expected {
                        left.replace(t1.unwrap())
                    } else {
                        right.replace(t2.unwrap())
                    }
                } else {
                    left.replace(t1.unwrap())
                }
            }
        }
    }

    pub fn replace(&self, type_: Type) -> Self {
        use TypeOutput::*;

        match self {
            Const(_) => ConstType(type_),
            ConstType(_) => ConstType(type_),
            Lit(_) => LitType(type_),
            LitType(_) => LitType(type_),
            Mut(_) => MutType(type_),
            MutType(_) => MutType(type_),
            None => None,
        }
    }

    pub fn replace_if_not_equal(&self, type_: Type) -> Self {
        use TypeOutput::*;

        match self {
            Const(v) if type_ != Type::from(v) => ConstType(type_),
            ConstType(t) if type_ != *t => ConstType(type_),
            Lit(v) if type_ != Type::from(v) => LitType(type_),
            LitType(t) if type_ != *t => LitType(type_),
            Mut(v) if type_ != Type::from(v) => MutType(type_),
            MutType(t) if type_ != *t => MutType(type_),
            correct => correct.clone(),
        }
    }

    pub fn is_const(&self) -> bool {
        matches!(self, Self::Const(_) | Self::Lit(_) | Self::Mut(_))
    }

    pub fn is_empty(&self) -> bool {
        matches!(self, Self::None)
    }
}

impl From<TypeOutput> for Option<Type> {
    fn from(t: TypeOutput) -> Self {
        t.as_ref().into()
    }
}

impl From<&TypeOutput> for Option<Type> {
    fn from(t: &TypeOutput) -> Self {
        match t {
            TypeOutput::Const(v) => Some(v.into()),
            TypeOutput::ConstType(t) => Some(*t),
            TypeOutput::Lit(v) => Some(v.into()),
            TypeOutput::LitType(t) => Some(*t),
            TypeOutput::Mut(v) => Some(v.into()),
            TypeOutput::MutType(t) => Some(*t),
            TypeOutput::None => None,
        }
    }
}

impl From<TypeOutput> for Option<Value> {
    fn from(t: TypeOutput) -> Self {
        t.as_ref().into()
    }
}

impl From<&TypeOutput> for Option<Value> {
    fn from(t: &TypeOutput) -> Self {
        if let TypeOutput::Const(v) = t {
            Some(v.clone())
        } else {
            None
        }
    }
}

impl From<Declaration> for TypeOutput {
    fn from(d: Declaration) -> Self {
        match d {
            Declaration::Const(Some(v)) => Self::Const(v),
            Declaration::Input(type_, ParamMode::Const) => Self::ConstType(type_),
            Declaration::Input(type_, _) => Self::MutType(type_),
            Declaration::Mut(Some(v)) => Self::Mut(v),
            _ => Self::None,
        }
    }
}

impl From<&Declaration> for TypeOutput {
    fn from(d: &Declaration) -> Self {
        match d {
            Declaration::Const(Some(v)) => Self::Const(v.clone()),
            Declaration::Input(type_, ParamMode::Const) => Self::ConstType(*type_),
            Declaration::Input(type_, _) => Self::MutType(*type_),
            Declaration::Mut(Some(v)) => Self::Mut(v.clone()),
            _ => Self::None,
        }
    }
}

impl From<&VariableSymbol> for TypeOutput {
    fn from(v: &VariableSymbol) -> Self {
        match &v.declaration {
            Declaration::Const(Some(v)) => Self::Const(v.clone()),
            Declaration::Const(None) => Self::ConstType(v.type_),
            Declaration::Input(type_, ParamMode::Const) => Self::ConstType(*type_),
            Declaration::Input(type_, _) => Self::MutType(*type_),
            Declaration::Mut(Some(v)) => Self::Mut(v.clone()),
            Declaration::Mut(None) => Self::MutType(v.type_),
        }
    }
}

impl From<Value> for TypeOutput {
    fn from(v: Value) -> Self {
        TypeOutput::Lit(v)
    }
}

impl From<&Value> for TypeOutput {
    fn from(v: &Value) -> Self {
        TypeOutput::Lit(v.clone())
    }
}

impl AsRef<Self> for TypeOutput {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl Default for TypeOutput {
    fn default() -> Self {
        Self::None
    }
}
