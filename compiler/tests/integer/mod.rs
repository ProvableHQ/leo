#[macro_use]
pub mod macros;
pub use self::macros::*;

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
}

// must be below macro definitions!
pub mod u32;
