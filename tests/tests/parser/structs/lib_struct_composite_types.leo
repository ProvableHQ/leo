// Parser test: library struct composite types with const generic arguments in
// various syntactic positions — function parameters, return types, local variable
// declarations, struct field types, and mapping value types.

struct Local::[N: u32] {
    v: u32,
}

program test.aleo {
    mapping cache: address => some_lib::Entry::[8u32];

    fn f(a: some_lib::Vec::[4u32], b: some_lib::Vec::[4u32]) -> some_lib::Vec::[4u32] {
        let x: some_lib::Vec::[4u32] = some_lib::Vec::[4u32] { size: 4u32 };
        let y: Local::[3u32] = Local::[3u32] { v: 0u32 };
        return x;
    }
}
