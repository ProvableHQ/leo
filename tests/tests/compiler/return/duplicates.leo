// As of commit 314d56a9e3626418e8a953ecb81f595555716459,
// programs with duplicate outputs did not work.
// Now they should clone the duplicates to satisfy SnarkVM.
struct S {
    x: field,
    y: field,
    z: field,
}

program test.aleo {
    fn foo1(x: field, y: field) -> (field, field, field, field, u8, u8) {
        return (x, x, y, y, 1u8, 1u8);
    }

    fn foo2(x: S, y: S) -> (S, S, field, field, u8, u8) {
        return (x, x, 2field, 2field, 1u8, 1u8);
    }
}
