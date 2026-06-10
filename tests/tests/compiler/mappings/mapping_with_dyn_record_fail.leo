program test.aleo {
    mapping bad_key: dyn record => u64;
    mapping bad_value: u64 => dyn record;

    fn main() -> u8 {
        return 0u8;
    }

    @noupgrade
    constructor() {}
}
