// Parser test: library struct with multiple const generic parameters in type positions.
// Verifies the parser correctly handles arity > 1 in composite type expressions.

program test.aleo {
    fn make(a: some_lib::Matrix::[2u32, 3u32]) -> some_lib::Matrix::[3u32, 4u32] {
        let m: some_lib::Matrix::[2u32, 3u32] = some_lib::Matrix::[2u32, 3u32] { r: 2u32, c: 3u32 };
        return some_lib::Matrix::[3u32, 4u32] { r: 3u32, c: 4u32 };
    }
}
