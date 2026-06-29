program test.aleo {
    mapping data: u32 => u8;
    mapping out: bool => u8;

    fn keep_after_assert(key: u32) -> Final {
        return final {
            let a: u8 = Mapping::get_or_use(data, key, 0u8);
            let b: u8 = Mapping::get_or_use(data, key, 0u8);
            assert_eq(b, a);
            let c: u8 = Mapping::get_or_use(data, key, 0u8);
            Mapping::set(out, false, b + c);
        };
    }

    @noupgrade
    constructor() {}
}
