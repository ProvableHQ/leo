program test.aleo {
    storage data: [u8];
    mapping result: bool => u8;

    fn read_one(idx: u32) -> Final {
        return final {
            let value: u8 = data.get(idx).unwrap_or(0u8);
            Mapping::set(result, false, value);
        };
    }

    @noupgrade
    constructor() {}
}
