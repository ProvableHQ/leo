
program test.aleo {
    fn bar(a: bool) -> [bool; 8] {
        return [a, a, a, a, a, a, a];
    }

    fn foo() -> [u8; 8] {
        return [1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u32];
    }
}
