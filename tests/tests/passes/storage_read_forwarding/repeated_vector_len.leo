program test.aleo {
    storage data: [u8];
    mapping out: bool => u32;

    fn reuse_len() -> Final {
        return final {
            let a: u32 = data.len();
            let b: u32 = data.len();
            Mapping::set(out, false, a + b);
        };
    }

    fn keep_after_push() -> Final {
        return final {
            let a: u32 = data.len();
            data.push(1u8);
            let b: u32 = data.len();
            Mapping::set(out, false, a + b);
        };
    }

    @noupgrade
    constructor() {}
}
