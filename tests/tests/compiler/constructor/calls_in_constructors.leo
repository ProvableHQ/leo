program test.aleo {
    mapping foo: u8 => u8;

    inline get_first_entry() -> u8 {
        foo.get(0u8)
    }
    
    inline check_first_entry_is_zero() {
        assert_eq(get_first_entry(), 0u8);
    }
    
    transition foo() {}
    
    async constructor() {
        check_first_entry_is_zero();
    }
}