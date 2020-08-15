//! Methods to enforce function input variables in a compiled Leo program.

pub mod array;
pub use self::array::*;

pub mod function_input;
pub use self::function_input::*;

pub mod main_function_input;
pub use self::main_function_input::*;

pub mod input_keyword;
pub use self::input_keyword::*;

pub mod input_section;
pub use self::input_section::*;

pub mod tuple;
pub use self::tuple::*;
