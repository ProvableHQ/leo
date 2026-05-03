program external_program.aleo {
    mapping balances: address => u64;

    storage var: u8;

    storage vec: [u8];

    fn touch() -> Final {
        return final {
            var = var.unwrap_or(0u8);
        };
    }

    @noupgrade
    constructor() {}
}
