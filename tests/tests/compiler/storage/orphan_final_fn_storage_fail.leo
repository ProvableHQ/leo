final fn orphan() {
    let value = flag.unwrap_or(0u8);
}

program orphan_final_fn_storage.aleo {
    @noupgrade
    constructor() {}

    storage flag: u8;

    fn main() -> u8 {
        return 0u8;
    }
}
