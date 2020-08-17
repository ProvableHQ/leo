#[macro_use]
pub mod int_macro;

#[macro_use]
pub mod uint_macro;

pub mod integer_tester;
pub use self::integer_tester::*;

// must be below macro definitions!
pub mod u128;
pub mod u16;
pub mod u32;
pub mod u64;
pub mod u8;

pub mod i128;
pub mod i16;
pub mod i32;
pub mod i64;
pub mod i8;
