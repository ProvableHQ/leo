program external_program.aleo {
    storage counter: u64;

    storage id_numbers: [u64];

    fn touch() -> Final {
        return final {
            counter = counter.unwrap_or(0u64) + 1u64;
        };
    }

    @noupgrade
    constructor() {}
}
