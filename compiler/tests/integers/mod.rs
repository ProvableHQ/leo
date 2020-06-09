#[macro_use]
pub mod macros;

use crate::{get_error, EdwardsTestCompiler};
use leo_compiler::errors::{CompilerError, FunctionError};
use leo_types::IntegerError;

pub trait IntegerTester {
    /// Tests use of the integer in a function input
    fn test_input();

    /// Tests a wrapping addition
    fn test_add();

    /// Tests a wrapping subtraction
    fn test_sub();

    /// Tests a wrapping multiplication
    fn test_mul();

    /// Tests a non-wrapping division
    fn test_div();

    /// Tests a wrapping exponentiation
    fn test_pow();

    /// Tests == evaluation
    fn test_eq();

    /// Tests >= evaluation
    fn test_ge();

    /// Tests > evaluation
    fn test_gt();

    /// Tests <= evaluation
    fn test_le();

    /// Tests < evaluation
    fn test_lt();

    /// Test assert equals constraint keyword
    fn test_assert_eq();

    /// Test ternary if bool ? num_1 : num_2;
    fn test_ternary();
}

pub(crate) fn fail_integer(program: EdwardsTestCompiler) {
    match get_error(program) {
        CompilerError::FunctionError(FunctionError::IntegerError(IntegerError::InvalidInteger(_string))) => {}
        error => panic!("Expected invalid boolean error, got {}", error),
    }
}

pub(crate) fn fail_synthesis(program: EdwardsTestCompiler) {
    match get_error(program) {
        CompilerError::FunctionError(FunctionError::IntegerError(IntegerError::SynthesisError(_string))) => {}
        error => panic!("Expected synthesis error, got {}", error),
    }
}

// must be below macro definitions!
// pub mod u128;
// pub mod u16;
pub mod u32;
// pub mod u64;
// pub mod u8;
