final fn update(value: u8) {
    flag = value;
}

program reachable_final_fn_storage.aleo {
    @noupgrade
    constructor() {}

    storage flag: u8;

    fn main(value: u8) -> Final {
        return final { update(value); };
    }
}
