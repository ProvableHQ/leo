program test.aleo {
    mapping foo: u8 => u8;

    inline check_first_entry_is_zero(entry: u8) {
        assert_eq(entry, 0u8);
    }
    
    transition food() {}

    @custom
    async constructor() {
        let entry: u8 = foo.get(0u8);
        check_first_entry_is_zero(entry);
    }
}
