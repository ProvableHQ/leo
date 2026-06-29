program test.aleo {
    mapping data: u32 => u8;
    mapping out: bool => u8;

    fn reuse_before_write(key: u32) -> Final {
        return final {
            let a: u8 = Mapping::get_or_use(data, key, 0u8);
            let b: u8 = Mapping::get_or_use(data, key, 0u8);
            Mapping::set(out, false, a + b);
        };
    }

    fn keep_after_write(key: u32) -> Final {
        return final {
            let a: u8 = Mapping::get_or_use(data, key, 0u8);
            Mapping::set(data, key, 1u8);
            let b: u8 = Mapping::get_or_use(data, key, 0u8);
            Mapping::set(out, false, a + b);
        };
    }

    fn reuse_contains(key: u32) -> Final {
        return final {
            let a: bool = Mapping::contains(data, key);
            let b: bool = Mapping::contains(data, key);
            let one: u8 = 1u8;
            let zero: u8 = 0u8;
            let value: u8 = a ? one : zero;
            Mapping::set(out, b, value);
        };
    }

    @noupgrade
    constructor() {}
}
