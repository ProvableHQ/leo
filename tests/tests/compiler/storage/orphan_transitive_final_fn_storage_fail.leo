final fn orphan() {
    update(1u8);
}

final fn update(value: u8) {
    flag = value;
}

program orphan_transitive_final_fn_storage.aleo {
    @noupgrade
    constructor() {}

    storage flag: u8;

    fn main() -> u8 {
        return 0u8;
    }
}
