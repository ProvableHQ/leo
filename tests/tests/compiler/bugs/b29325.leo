// Regression test for https://github.com/ProvableHQ/leo/issues/29325.
// The compiler previously panicked with `ParseIntError { kind: PosOverflow }`
// when writing to an element of an array of size > 256 using a u8 index,
// because the SSA write-transforming pass created U8 index literals that
// overflow for indices >= 256.
program test.aleo {
    fn foo(t: [u8; 257]) -> [u8; 257] {
        t[0u8] = 1u8;
        return t;
    }
}
