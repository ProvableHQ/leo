program first_public_storage.aleo {
    storage count: u8;

    fn increment() -> Final {
        return final {
            count = count.unwrap_or(0u8) + 1u8;
        };
    }

    @noupgrade
    constructor() {}
}
