//! Imports are split up into two parts: parsing and storing

/// The import parser creates a hashmap of import program names -> import program structs
pub mod parser;
pub use self::parser::*;

/// The import store brings an imported symbol into the main program from an import program struct
pub mod store;
pub use self::store::*;
