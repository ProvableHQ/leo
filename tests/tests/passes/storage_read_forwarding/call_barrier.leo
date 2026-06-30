fn identity(x: u8) -> u8 {
    return x;
}

program test.aleo {
    mapping data: u32 => u8;
    mapping out: bool => u8;

    fn keep_after_call(key: u32) -> Final {
        return final {
            let a: u8 = Mapping::get_or_use(data, key, 0u8);
            let touched: u8 = identity(a);
            let b: u8 = Mapping::get_or_use(data, key, 0u8);
            Mapping::set(out, false, touched + b);
        };
    }

    @noupgrade
    constructor() {}
}
