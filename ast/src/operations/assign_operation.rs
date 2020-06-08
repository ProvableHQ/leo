use crate::ast::Rule;

use pest_ast::FromPest;

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::operation_assign))]
pub enum AssignOperation {
    Assign(Assign),
    AddAssign(AddAssign),
    SubAssign(SubAssign),
    MulAssign(MulAssign),
    DivAssign(DivAssign),
    PowAssign(PowAssign),
}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::assign))]
pub struct Assign {}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::operation_add_assign))]
pub struct AddAssign {}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::operation_sub_assign))]
pub struct SubAssign {}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::operation_mul_assign))]
pub struct MulAssign {}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::operation_div_assign))]
pub struct DivAssign {}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::operation_pow_assign))]
pub struct PowAssign {}
