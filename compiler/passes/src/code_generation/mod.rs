// Copyright (C) 2019-2026 Provable Inc.
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

use crate::Pass;

use itertools::Itertools;
use leo_ast::{CompiledPrograms, Mode, ProgramId};
use leo_errors::Result;

use std::fmt::Display;

mod expression;

mod program;

mod statement;

mod type_;

mod visitor;
use snarkvm::{
    prelude::{ArrayType, LiteralType, Network, PlaintextType},
    synthesizer::program::{CommitVariant, DeserializeVariant, ECDSAVerifyVariant, HashVariant, SerializeVariant},
};
use visitor::*;

pub struct CodeGenerating;

impl Pass for CodeGenerating {
    type Input = ();
    type Output = CompiledPrograms;

    const NAME: &str = "CodeGenerating";

    fn do_pass(_input: Self::Input, state: &mut crate::CompilerState) -> Result<Self::Output> {
        let mut visitor = CodeGeneratingVisitor {
            state,
            next_register: 0,
            current_function: None,
            variable_mapping: Default::default(),
            composite_mapping: Default::default(),
            global_mapping: Default::default(),
            variant: None,
            program: &state.ast.ast,
            program_id: None,
            finalize_caller: None,
            next_label: 0,
            conditional_depth: 0,
            internal_record_inputs: Default::default(),
        };

        Ok(visitor.visit_package())
    }
}

#[derive(Debug)]
pub struct AleoProgram {
    imports: Vec<String>,
    program_id: ProgramId,
    data_types: Vec<AleoDatatype>, // order matters
    mappings: Vec<AleoMapping>,
    functions: Vec<AleoFunctional>,
    constructor: Option<AleoConstructor>,
}

impl Display for AleoProgram {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.imports
            .iter()
            .map(|program_name| format!("import {program_name}.aleo;"))
            .chain(std::iter::once(format!("program {};\n", self.program_id)))
            .chain(self.data_types.iter().map(ToString::to_string))
            .chain(self.mappings.iter().map(ToString::to_string))
            .chain(self.functions.iter().map(ToString::to_string))
            .chain(self.constructor.iter().map(ToString::to_string))
            .join("\n")
            .fmt(f)
    }
}

#[derive(Debug)]
pub enum AleoDatatype {
    Struct(AleoStruct),
    Record(AleoRecord),
}

impl Display for AleoDatatype {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Struct(s) => s.fmt(f),
            Self::Record(r) => r.fmt(f),
        }
    }
}

#[derive(Debug)]
pub struct AleoStruct {
    name: String,
    fields: Vec<(String, AleoType)>,
}
impl Display for AleoStruct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "struct {}:", self.name)?;
        for (id, ty) in self.fields.iter() {
            writeln!(f, "    {id} as {ty};")?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct AleoRecord {
    name: String,
    fields: Vec<(String, AleoType, AleoVisibility)>,
}
impl Display for AleoRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "record {}:", self.name)?;
        for (id, ty, mode) in self.fields.iter() {
            writeln!(f, "    {id} as {ty}.{mode};")?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum AleoVisibility {
    Constant,
    Public,
    Private,
}
impl AleoVisibility {
    fn maybe_from(mode: Mode) -> Option<Self> {
        match mode {
            Mode::None => None,
            Mode::Constant => Some(Self::Constant),
            Mode::Private => Some(Self::Private),
            Mode::Public => Some(Self::Public),
        }
    }
}

impl Display for AleoVisibility {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::Constant => "constant",
            Self::Public => "public",
            Self::Private => "private",
        })
    }
}

#[derive(Debug)]
pub struct AleoMapping {
    name: String,
    key_type: AleoType,
    key_visibility: Option<AleoVisibility>,
    value_type: AleoType,
    value_visibility: Option<AleoVisibility>,
}
impl Display for AleoMapping {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "mapping {}:", self.name)?;
        if let Some(key_viz) = &self.key_visibility {
            writeln!(f, "    key as {}.{};", self.key_type, key_viz)?;
        } else {
            writeln!(f, "    key as {};", self.key_type)?;
        }
        if let Some(value_viz) = &self.value_visibility {
            writeln!(f, "    value as {}.{};", self.value_type, value_viz)?;
        } else {
            writeln!(f, "    value as {};", self.value_type)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct AleoClosure {
    name: String,
    inputs: Vec<AleoInput>,
    statements: Vec<AleoStmt>,
}
impl Display for AleoClosure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "closure {}:", self.name)?;
        for input in &self.inputs {
            write!(f, "{}", input)?;
        }
        for stm in &self.statements {
            write!(f, "{}", stm)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum AleoFunctional {
    Closure(AleoClosure),
    Function(AleoFunction),
    Finalize(AleoFinalize),
}
impl AleoFunctional {
    pub fn as_closure(self) -> AleoClosure {
        if let Self::Closure(c) = self { c } else { panic!("this is not a closure") }
    }

    pub fn as_function(self) -> AleoFunction {
        if let Self::Function(c) = self { c } else { panic!("this is not a function") }
    }

    pub fn as_function_ref_mut(&mut self) -> &mut AleoFunction {
        if let Self::Function(c) = self { c } else { panic!("this is not a function") }
    }

    pub fn as_finalize(self) -> AleoFinalize {
        if let Self::Finalize(c) = self { c } else { panic!("this is not a finalize") }
    }
}
impl Display for AleoFunctional {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Closure(c) => c.fmt(f),
            Self::Function(fun) => fun.fmt(f),
            Self::Finalize(fin) => fin.fmt(f),
        }
    }
}

#[derive(Debug)]
pub struct AleoFunction {
    name: String,
    inputs: Vec<AleoInput>,
    statements: Vec<AleoStmt>,
    finalize: Option<AleoFinalize>,
}
impl Display for AleoFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "function {}:", self.name)?;
        for input in &self.inputs {
            write!(f, "{}", input)?;
        }
        for stm in &self.statements {
            write!(f, "{}", stm)?;
        }
        if let Some(finalize) = &self.finalize {
            write!(f, "\n{}", finalize)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct AleoFinalize {
    caller_name: String,
    inputs: Vec<AleoInput>,
    statements: Vec<AleoStmt>,
}
impl Display for AleoFinalize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "finalize {}:", self.caller_name)?;
        for input in &self.inputs {
            write!(f, "{}", input)?;
        }
        for stm in &self.statements {
            write!(f, "{}", stm)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct AleoInput {
    register: AleoReg,
    type_: AleoType,
    visibility: Option<AleoVisibility>,
}
impl Display for AleoInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(visibility) = &self.visibility {
            writeln!(f, "    input {} as {}.{};", self.register, self.type_, visibility)
        } else {
            writeln!(f, "    input {} as {};", self.register, self.type_)
        }
    }
}

#[derive(Debug)]
pub struct AleoConstructor {
    statements: Vec<AleoStmt>,
}
impl Display for AleoConstructor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "constructor:")?;
        for stm in &self.statements {
            write!(f, "{}", stm)?;
        }
        Ok(())
    }
}

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub enum AleoExpr {
    Reg(AleoReg),
    Tuple(Vec<AleoExpr>),
    ArrayAccess(Box<AleoExpr>, Box<AleoExpr>),
    MemberAccess(Box<AleoExpr>, String),
    RawName(String),
    // Literals
    Address(String),
    Bool(bool),
    Field(String),
    Group(String),
    Scalar(String),
    String(String),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
}
impl Display for AleoExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Reg(reg) => write!(f, "{reg}"),
            Self::Tuple(regs) => write!(f, "{}", regs.iter().map(|reg| reg.to_string()).join(" ")),
            Self::ArrayAccess(array, index) => write!(f, "{array}[{index}]"),
            Self::MemberAccess(comp, member) => write!(f, "{comp}.{member}"),
            Self::RawName(n) => write!(f, "{n}"),
            Self::Address(val) => write!(f, "{val}"),
            Self::Bool(val) => write!(f, "{val}"),
            Self::Field(val) => write!(f, "{val}field"),
            Self::Group(val) => write!(f, "{val}group"),
            Self::Scalar(val) => write!(f, "{val}scalar"),
            Self::String(val) => write!(f, "\"{val}\""),
            Self::U8(val) => write!(f, "{val}u8"),
            Self::U16(val) => write!(f, "{val}u16"),
            Self::U32(val) => write!(f, "{val}u32"),
            Self::U64(val) => write!(f, "{val}u64"),
            Self::U128(val) => write!(f, "{val}u128"),
            Self::I8(val) => write!(f, "{val}i8"),
            Self::I16(val) => write!(f, "{val}i16"),
            Self::I32(val) => write!(f, "{val}i32"),
            Self::I64(val) => write!(f, "{val}i64"),
            Self::I128(val) => write!(f, "{val}i128"),
        }
    }
}

#[derive(Debug)]
pub enum AleoStmt {
    Output(AleoExpr, AleoType, Option<AleoVisibility>),
    AssertEq(AleoExpr, AleoExpr),
    AssertNeq(AleoExpr, AleoExpr),
    Cast(AleoExpr, AleoReg, AleoType),
    Abs(AleoExpr, AleoReg),
    AbsW(AleoExpr, AleoReg),
    Double(AleoExpr, AleoReg),
    Inv(AleoExpr, AleoReg),
    Not(AleoExpr, AleoReg),
    Neg(AleoExpr, AleoReg),
    Square(AleoExpr, AleoReg),
    Sqrt(AleoExpr, AleoReg),
    Ternary(AleoExpr, AleoExpr, AleoExpr, AleoReg),
    Commit(CommitVariant, AleoExpr, AleoExpr, AleoReg, AleoType),
    Hash(HashVariant, AleoExpr, AleoReg, AleoType),
    Get(AleoExpr, AleoExpr, AleoReg),
    GetOrUse(AleoExpr, AleoExpr, AleoExpr, AleoReg),
    Set(AleoExpr, AleoExpr, AleoExpr),
    Remove(AleoExpr, AleoExpr),
    Contains(AleoExpr, AleoExpr, AleoReg),
    RandChacha(AleoReg, AleoType),
    SignVerify(AleoExpr, AleoExpr, AleoExpr, AleoReg),
    EcdsaVerify(ECDSAVerifyVariant, AleoExpr, AleoExpr, AleoExpr, AleoReg),
    Await(AleoExpr),
    Serialize(SerializeVariant, AleoExpr, AleoType, AleoReg, AleoType),
    Deserialize(DeserializeVariant, AleoExpr, AleoType, AleoReg, AleoType),
    Call(String, Vec<AleoExpr>, Vec<AleoReg>),
    Async(String, Vec<AleoExpr>, Vec<AleoReg>),
    BranchEq(AleoExpr, AleoExpr, String),
    Position(String),
    Add(AleoExpr, AleoExpr, AleoReg),
    AddWrapped(AleoExpr, AleoExpr, AleoReg),
    And(AleoExpr, AleoExpr, AleoReg),
    Div(AleoExpr, AleoExpr, AleoReg),
    DivWrapped(AleoExpr, AleoExpr, AleoReg),
    Eq(AleoExpr, AleoExpr, AleoReg),
    Gte(AleoExpr, AleoExpr, AleoReg),
    Gt(AleoExpr, AleoExpr, AleoReg),
    Lte(AleoExpr, AleoExpr, AleoReg),
    Lt(AleoExpr, AleoExpr, AleoReg),
    Mod(AleoExpr, AleoExpr, AleoReg),
    Mul(AleoExpr, AleoExpr, AleoReg),
    MulWrapped(AleoExpr, AleoExpr, AleoReg),
    Nand(AleoExpr, AleoExpr, AleoReg),
    Neq(AleoExpr, AleoExpr, AleoReg),
    Nor(AleoExpr, AleoExpr, AleoReg),
    Or(AleoExpr, AleoExpr, AleoReg),
    Pow(AleoExpr, AleoExpr, AleoReg),
    PowWrapped(AleoExpr, AleoExpr, AleoReg),
    Rem(AleoExpr, AleoExpr, AleoReg),
    RemWrapped(AleoExpr, AleoExpr, AleoReg),
    Shl(AleoExpr, AleoExpr, AleoReg),
    ShlWrapped(AleoExpr, AleoExpr, AleoReg),
    Shr(AleoExpr, AleoExpr, AleoReg),
    ShrWrapped(AleoExpr, AleoExpr, AleoReg),
    Sub(AleoExpr, AleoExpr, AleoReg),
    SubWrapped(AleoExpr, AleoExpr, AleoReg),
    Xor(AleoExpr, AleoExpr, AleoReg),
}
impl Display for AleoStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Output(arg, type_, viz) => {
                if let Some(viz) = &viz {
                    writeln!(f, "    output {} as {}.{};", arg, type_, viz)
                } else {
                    writeln!(f, "    output {} as {};", arg, type_)
                }
            }
            Self::AssertEq(left, right) => writeln!(f, "    assert.eq {left} {right};"),
            Self::AssertNeq(left, right) => writeln!(f, "    assert.neq {left} {right};"),
            Self::Cast(operands, dest, type_) => {
                writeln!(f, "    cast {operands} into {dest} as {type_};")
            }
            Self::Abs(op, dest) => writeln!(f, "    abs {op} into {dest};"),
            Self::AbsW(op, dest) => writeln!(f, "    abs.w {op} into {dest};"),
            Self::Double(op, dest) => writeln!(f, "    double {op} into {dest};"),
            Self::Inv(op, dest) => writeln!(f, "    inv {op} into {dest};"),
            Self::Not(op, dest) => writeln!(f, "    not {op} into {dest};"),
            Self::Neg(op, dest) => writeln!(f, "    neg {op} into {dest};"),
            Self::Square(op, dest) => writeln!(f, "    square {op} into {dest};"),
            Self::Sqrt(op, dest) => writeln!(f, "    sqrt {op} into {dest};"),
            Self::Ternary(cond, if_true, if_false, dest) => {
                writeln!(f, "    ternary {cond} {if_true} {if_false} into {dest};")
            }
            Self::Commit(variant, arg0, arg1, dest, type_) => {
                writeln!(
                    f,
                    "    {} {arg0} {arg1} into {dest} as {type_};",
                    CommitVariant::opcode(variant.clone() as u8)
                )
            }
            Self::Hash(variant, arg0, dest, type_) => {
                writeln!(f, "    {} {arg0} into {dest} as {type_};", variant.opcode())
            }
            Self::Get(mapping, key, dest) => writeln!(f, "    get {mapping}[{key}] into {dest};"),
            Self::GetOrUse(mapping, key, default, dest) => {
                writeln!(f, "    get.or_use {mapping}[{key}] {default} into {dest};")
            }
            Self::Set(value, mapping, key) => writeln!(f, "    set {value} into {mapping}[{key}];"),
            Self::Remove(mapping, key) => writeln!(f, "    remove {mapping}[{key}];"),
            Self::Contains(mapping, key, dest) => writeln!(f, "    contains {mapping}[{key}] into {dest};"),
            Self::RandChacha(dest, type_) => writeln!(f, "    rand.chacha into {dest} as {type_};"),
            Self::SignVerify(arg0, arg1, arg2, dest) => {
                writeln!(f, "    sign.verify {arg0} {arg1} {arg2} into {dest};")
            }
            Self::EcdsaVerify(variant, arg0, arg1, arg2, dest) => {
                writeln!(f, "    {} {arg0} {arg1} {arg2} into {dest};", variant.opcode())
            }
            Self::Await(exp) => writeln!(f, "    await {exp};"),
            Self::Serialize(variant, input, input_ty, dest, dest_ty) => {
                writeln!(
                    f,
                    "    {} {input} ({input_ty}) into {dest} ({dest_ty});",
                    SerializeVariant::opcode(variant.clone() as u8),
                )
            }
            Self::Deserialize(variant, input, input_ty, dest, dest_ty) => {
                writeln!(
                    f,
                    "    {} {input} ({input_ty}) into {dest} ({dest_ty});",
                    DeserializeVariant::opcode(variant.clone() as u8),
                )
            }
            Self::Call(id, inputs, dests) => {
                write!(f, "    call {id}")?;
                if !inputs.is_empty() {
                    write!(f, " {}", inputs.iter().map(|input| input.to_string()).join(" "))?;
                }
                if !dests.is_empty() {
                    write!(f, " into {}", dests.iter().map(|input| input.to_string()).join(" "))?;
                }
                writeln!(f, ";")
            }
            Self::Async(id, inputs, dests) => {
                write!(f, "    async {id}")?;
                if !inputs.is_empty() {
                    write!(f, " {}", inputs.iter().map(|input| input.to_string()).join(" "))?;
                }
                if !dests.is_empty() {
                    write!(f, " into {}", dests.iter().map(|input| input.to_string()).join(" "))?;
                }
                writeln!(f, ";")
            }
            Self::BranchEq(arg0, arg1, label) => {
                writeln!(f, "    branch.eq {arg0} {arg1} to {label};")
            }
            Self::Position(label) => writeln!(f, "    position {label};"),
            Self::Add(arg0, arg1, dest) => writeln!(f, "    add {arg0} {arg1} into {dest};"),
            Self::AddWrapped(arg0, arg1, dest) => writeln!(f, "    add.w {arg0} {arg1} into {dest};"),
            Self::And(arg0, arg1, dest) => writeln!(f, "    and {arg0} {arg1} into {dest};"),
            Self::Div(arg0, arg1, dest) => writeln!(f, "    div {arg0} {arg1} into {dest};"),
            Self::DivWrapped(arg0, arg1, dest) => writeln!(f, "    div.w {arg0} {arg1} into {dest};"),
            Self::Eq(arg0, arg1, dest) => writeln!(f, "    is.eq {arg0} {arg1} into {dest};"),
            Self::Gte(arg0, arg1, dest) => writeln!(f, "    gte {arg0} {arg1} into {dest};"),
            Self::Gt(arg0, arg1, dest) => writeln!(f, "    gt {arg0} {arg1} into {dest};"),
            Self::Lte(arg0, arg1, dest) => writeln!(f, "    lte {arg0} {arg1} into {dest};"),
            Self::Lt(arg0, arg1, dest) => writeln!(f, "    lt {arg0} {arg1} into {dest};"),
            Self::Mod(arg0, arg1, dest) => writeln!(f, "    mod {arg0} {arg1} into {dest};"),
            Self::Mul(arg0, arg1, dest) => writeln!(f, "    mul {arg0} {arg1} into {dest};"),
            Self::MulWrapped(arg0, arg1, dest) => writeln!(f, "    mul.w {arg0} {arg1} into {dest};"),
            Self::Nand(arg0, arg1, dest) => writeln!(f, "    nand {arg0} {arg1} into {dest};"),
            Self::Neq(arg0, arg1, dest) => writeln!(f, "    is.neq {arg0} {arg1} into {dest};"),
            Self::Nor(arg0, arg1, dest) => writeln!(f, "    nor {arg0} {arg1} into {dest};"),
            Self::Or(arg0, arg1, dest) => writeln!(f, "    or {arg0} {arg1} into {dest};"),
            Self::Pow(arg0, arg1, dest) => writeln!(f, "    pow {arg0} {arg1} into {dest};"),
            Self::PowWrapped(arg0, arg1, dest) => writeln!(f, "    pow.w {arg0} {arg1} into {dest};"),
            Self::Rem(arg0, arg1, dest) => writeln!(f, "    rem {arg0} {arg1} into {dest};"),
            Self::RemWrapped(arg0, arg1, dest) => writeln!(f, "    rem.w {arg0} {arg1} into {dest};"),
            Self::Shl(arg0, arg1, dest) => writeln!(f, "    shl {arg0} {arg1} into {dest};"),
            Self::ShlWrapped(arg0, arg1, dest) => writeln!(f, "    shl.w {arg0} {arg1} into {dest};"),
            Self::Shr(arg0, arg1, dest) => writeln!(f, "    shr {arg0} {arg1} into {dest};"),
            Self::ShrWrapped(arg0, arg1, dest) => writeln!(f, "    shr.w {arg0} {arg1} into {dest};"),
            Self::Sub(arg0, arg1, dest) => writeln!(f, "    sub {arg0} {arg1} into {dest};"),
            Self::SubWrapped(arg0, arg1, dest) => writeln!(f, "    sub.w {arg0} {arg1} into {dest};"),
            Self::Xor(arg0, arg1, dest) => writeln!(f, "    xor {arg0} {arg1} into {dest};"),
        }
    }
}

#[derive(Debug)]
pub enum AleoType {
    Future { name: String, program: String },
    Record { name: String, program: Option<String> },
    Ident { name: String },
    Location { program: String, name: String },
    Array { inner: Box<AleoType>, len: u32 },
    GroupX,
    GroupY,
    Address,
    Boolean,
    Field,
    Group,
    I8,
    I16,
    I32,
    I64,
    I128,
    U8,
    U16,
    U32,
    U64,
    U128,
    Scalar,
    Signature,
    String,
}
impl Display for AleoType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Future { name, program } => write!(f, "{program}.aleo/{name}.future"),
            Self::Record { name, program: Some(program_name) } => write!(f, "{program_name}.aleo/{name}.record"),
            Self::Record { name, program: None } => write!(f, "{name}.record"),
            Self::GroupX => write!(f, "group.x"),
            Self::GroupY => write!(f, "group.y"),
            Self::Ident { name } => write!(f, "{name}"),
            Self::Location { program, name } => write!(f, "{program}.aleo/{name}"),
            Self::Address => write!(f, "address"),
            Self::Boolean => write!(f, "boolean"),
            Self::Field => write!(f, "field"),
            Self::Group => write!(f, "group"),
            Self::I8 => write!(f, "i8"),
            Self::I16 => write!(f, "i16"),
            Self::I32 => write!(f, "i32"),
            Self::I64 => write!(f, "i64"),
            Self::I128 => write!(f, "i128"),
            Self::U8 => write!(f, "u8"),
            Self::U16 => write!(f, "u16"),
            Self::U32 => write!(f, "u32"),
            Self::U64 => write!(f, "u64"),
            Self::U128 => write!(f, "u128"),
            Self::Scalar => write!(f, "scalar"),
            Self::Signature => write!(f, "signature"),
            Self::String => write!(f, "string"),
            Self::Array { inner, len } => write!(f, "[{inner}; {len}u32]"),
        }
    }
}
impl<N: Network> From<PlaintextType<N>> for AleoType {
    fn from(value: PlaintextType<N>) -> Self {
        match value {
            PlaintextType::Literal(lit) => lit.into(),
            PlaintextType::Struct(id) => Self::Ident { name: id.to_string() },
            PlaintextType::ExternalStruct(loc) => {
                Self::Location { program: loc.program_id().to_string(), name: loc.name().to_string() }
            }
            PlaintextType::Array(arr) => arr.into(),
        }
    }
}
impl From<LiteralType> for AleoType {
    fn from(value: LiteralType) -> Self {
        match value {
            LiteralType::Address => Self::Address,
            LiteralType::Boolean => Self::Boolean,
            LiteralType::Field => Self::Field,
            LiteralType::Group => Self::Group,
            LiteralType::I8 => Self::I8,
            LiteralType::I16 => Self::I16,
            LiteralType::I32 => Self::I32,
            LiteralType::I64 => Self::I64,
            LiteralType::I128 => Self::I128,
            LiteralType::U8 => Self::U8,
            LiteralType::U16 => Self::U16,
            LiteralType::U32 => Self::U32,
            LiteralType::U64 => Self::U64,
            LiteralType::U128 => Self::U128,
            LiteralType::Scalar => Self::Scalar,
            LiteralType::Signature => Self::Signature,
            LiteralType::String => Self::String,
        }
    }
}
impl<N: Network> From<ArrayType<N>> for AleoType {
    fn from(value: ArrayType<N>) -> Self {
        Self::Array { len: **value.length(), inner: Box::new(AleoType::from(value.next_element_type().clone())) }
    }
}

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub enum AleoReg {
    Self_,
    Block,
    Network,
    R(u64),
}
impl Display for AleoReg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Self_ => write!(f, "self"),
            Self::Block => write!(f, "block"),
            Self::Network => write!(f, "network"),
            Self::R(n) => write!(f, "r{n}"),
        }
    }
}
