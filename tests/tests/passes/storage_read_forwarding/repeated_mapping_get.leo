program test.aleo {
    mapping data: u32 => u8;
    mapping out: bool => u8;

    fn reuse_get(key: u32) -> Final {
        return final {
            let a: u8 = Mapping::get(data, key);
            let b: u8 = Mapping::get(data, key);
            Mapping::set(out, false, a + b);
        };
    }

    fn keep_get_after_set(key: u32) -> Final {
        return final {
            let a: u8 = Mapping::get(data, key);
            Mapping::set(data, key, 1u8);
            let b: u8 = Mapping::get(data, key);
            Mapping::set(out, false, a + b);
        };
    }

    fn keep_get_after_remove(key: u32) -> Final {
        return final {
            let a: u8 = Mapping::get(data, key);
            Mapping::remove(data, key);
            let b: u8 = Mapping::get(data, key);
            Mapping::set(out, false, a + b);
        };
    }

    @noupgrade
    constructor() {}
}
