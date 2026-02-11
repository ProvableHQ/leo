struct S {
    member: Final,
}

program test.aleo {
    mapping x: Final => Final;
    fn main() -> Final {
        let future = final {
            assert_eq(1u8, 1u8);
        };
        let an_array: [Final; 1] = [future];
        return an_array[0u32];
    }
}
