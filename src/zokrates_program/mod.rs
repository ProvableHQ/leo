//! Module containing structs and types that make up a zokrates_program.
//!
//! @file zokrates_program.rs
//! @author Collin Chin <collin@aleo.org>
//! @date 2020

pub mod program;
pub use self::program::*;

pub mod types;
pub use self::types::*;

pub mod types_display;
pub use self::types_display::*;

pub mod types_from;
pub use self::types_from::*;
