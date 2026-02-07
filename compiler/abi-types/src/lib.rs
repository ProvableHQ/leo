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

//! ABI type definitions for Leo programs.
//!
//! This crate provides types that describe the public interface of a Leo program,
//! including transitions, mappings, and all related types. The ABI enables downstream
//! tooling to interact with deployed Leo programs.
//!
//! # Lowered Types
//!
//! Some Leo types have an alternative "lowered" form in the compiled Aleo bytecode.
//! Downstream tooling should apply these transformations to understand the on-chain
//! representation:
//!
//! - [`Optional`] - Lowered to a struct with `is_some: bool` and `val: T` fields.

use serde::{Deserialize, Serialize};

/// A path to a type (e.g., `["utils", "math", "Vector3"]` for `utils::math::Vector3`).
pub type Path = Vec<String>;

/// The complete ABI for a Leo program.
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Program {
    /// The program identifier (e.g., "token.aleo").
    pub program: String,
    /// Struct type definitions.
    pub structs: Vec<Struct>,
    /// Record type definitions.
    pub records: Vec<Record>,
    /// On-chain key-value storage definitions.
    pub mappings: Vec<Mapping>,
    /// Storage variable definitions.
    pub storage_variables: Vec<StorageVariable>,
    /// Public entry points (transitions only, not internal functions).
    pub transitions: Vec<Transition>,
}

/// A struct type definition.
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Struct {
    /// Path to the struct (e.g., `["Point"]` or `["utils", "Vector3"]` for module structs).
    pub path: Path,
    pub fields: Vec<StructField>,
}

/// A record type definition. Records have an implicit `owner: address` field.
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Record {
    /// Path to the record (e.g., `["Token"]` or `["utils", "Token"]` for module records).
    pub path: Path,
    pub fields: Vec<RecordField>,
}

/// An on-chain key-value mapping.
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Mapping {
    pub name: String,
    pub key: Plaintext,
    pub value: Plaintext,
}

/// A storage variable declaration.
///
/// # Lowering
///
/// Storage variables are lowered to mappings in Aleo bytecode:
/// - `storage x: T` becomes `mapping x__: bool => T` (value stored at key `false`)
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct StorageVariable {
    pub name: String,
    pub ty: StorageType,
}

/// Type for storage variables. Supports Vector unlike Plaintext.
///
/// # Lowering
///
/// Storage vectors are lowered to two mappings:
/// - `storage vec: Vector<T>` becomes:
///   - `mapping vec__: u32 => T` (elements by index)
///   - `mapping vec__len__: bool => u32` (length at key `false`)
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum StorageType {
    Plaintext(Plaintext),
    Vector(Box<StorageType>),
}

/// A transition function (public entry point).
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Transition {
    pub name: String,
    pub is_async: bool,
    pub inputs: Vec<Input>,
    pub outputs: Vec<Output>,
}

/// A struct field.
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct StructField {
    pub name: String,
    pub ty: Plaintext,
}

/// A record field with visibility mode.
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct RecordField {
    pub name: String,
    pub ty: Plaintext,
    pub mode: Mode,
}

/// A transition input.
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Input {
    pub name: String,
    pub ty: TransitionInput,
    pub mode: Mode,
}

/// A transition output.
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Output {
    pub ty: TransitionOutput,
    pub mode: Mode,
}

/// Visibility mode for inputs, outputs, and record fields.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]
pub enum Mode {
    None,
    Constant,
    Private,
    Public,
}

/// A fixed-length array type.
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Array {
    pub element: Box<Plaintext>,
    pub length: u32,
}

/// A reference to a struct type, possibly from another program.
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct StructRef {
    /// Path segments to the struct (e.g., `["utils", "Vector3"]` for `utils::Vector3`).
    pub path: Path,
    /// The program containing this struct, if external.
    pub program: Option<String>,
}

/// A reference to a record type, possibly from another program.
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct RecordRef {
    /// Path segments to the record (e.g., `["Token"]` for a top-level record).
    pub path: Path,
    /// The program containing this record, if external.
    pub program: Option<String>,
}

/// An optional type (`T?`).
///
/// # Lowering
///
/// In the compiled Aleo bytecode, `T?` is lowered to a struct:
/// ```text
/// struct "T?" { is_some: bool, val: T }
/// ```
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Optional(pub Box<Plaintext>);

/// A plaintext type (not encrypted). Used for struct fields, mapping keys/values, etc.
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum Plaintext {
    Primitive(Primitive),
    Array(Array),
    Struct(StructRef),
    Optional(Optional),
}

/// Valid types for transition inputs.
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum TransitionInput {
    Plaintext(Plaintext),
    Record(RecordRef),
}

/// Valid types for transition outputs.
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum TransitionOutput {
    Plaintext(Plaintext),
    Record(RecordRef),
    /// A future returned by async transitions.
    Future,
}

/// Primitive types that map directly to Aleo literal types.
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum Primitive {
    Address,
    Boolean,
    Field,
    Group,
    Scalar,
    Signature,
    Int(Int),
    UInt(UInt),
}

/// Signed integer types.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]
pub enum Int {
    I8,
    I16,
    I32,
    I64,
    I128,
}

/// Unsigned integer types.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]
pub enum UInt {
    U8,
    U16,
    U32,
    U64,
    U128,
}
