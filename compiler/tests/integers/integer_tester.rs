use crate::{expect_compiler_error, EdwardsTestCompiler};
use leo_compiler::errors::{CompilerError, ExpressionError, FunctionError, IntegerError, StatementError, ValueError};

pub trait IntegerTester {
    /// Tests defining the smalled value that can be represented by the integer type
    fn test_min();

    /// Tests defining the smallest value - 1
    fn test_min_fail();

    /// Tests defining the largest value that can be represented by the integer type
    fn test_max();

    /// Tests defining the largest value + 1
    fn test_max_fail();

    /// Tests a non-wrapping addition
    fn test_add();

    /// Tests a non-wrapping subtraction
    fn test_sub();

    /// Tests a non-wrapping multiplication
    fn test_mul();

    /// Tests a non-wrapping division
    fn test_div();

    /// Tests a non-wrapping exponentiation
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

pub(crate) fn expect_parsing_error(program: EdwardsTestCompiler) {
    match expect_compiler_error(program) {
        CompilerError::FunctionError(FunctionError::StatementError(StatementError::ExpressionError(
            ExpressionError::ValueError(ValueError::IntegerError(IntegerError::Error(_))),
        ))) => {}
        error => panic!("Expected integer parsing error, found {:?}", error),
    }
}

pub(crate) fn expect_computation_error(program: EdwardsTestCompiler) {
    match expect_compiler_error(program) {
        CompilerError::FunctionError(FunctionError::StatementError(StatementError::ExpressionError(
            ExpressionError::IntegerError(IntegerError::Error(_)),
        ))) => {}
        error => panic!(
            "Expected integer computation error such as `DivisionByZero`, found {:?}",
            error
        ),
    }
}
