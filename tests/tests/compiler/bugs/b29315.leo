program test.aleo {
    mapping m: u32 => u32;

    fn foo() -> Final {
        return final { assert_eq(Mapping::set(m, 0u32, 0u32), Mapping::set(m, 0u32, 0u32)); };
    }
}
