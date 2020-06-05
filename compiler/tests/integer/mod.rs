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

// must be below macro definitions!
// pub mod u8;
pub mod u32;
