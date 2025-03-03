/*
namespace = "Compile"
expectation = "Pass"
*/

program test.aleo {
    struct S {
        x: u8,
        y: u8,
    }

    transition main() -> u8 {
        const s: S = S { x: 1u8, y: 2u8 };
        return s.x;
    }
}
