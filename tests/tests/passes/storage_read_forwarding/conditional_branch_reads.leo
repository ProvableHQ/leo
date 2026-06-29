program test.aleo {
    mapping data: u32 => u8;
    mapping out: bool => u8;

    fn branch_local(key: u32, flag: bool) -> Final {
        return final {
            if flag {
                let a: u8 = Mapping::get_or_use(data, key, 0u8);
                let b: u8 = Mapping::get_or_use(data, key, 0u8);
                Mapping::set(out, false, a + b);
            }

            let c: u8 = Mapping::get_or_use(data, key, 0u8);
            Mapping::set(out, true, c);
        };
    }

    @noupgrade
    constructor() {}
}
