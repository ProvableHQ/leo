program other_program.aleo {
    storage hits: u64;

    fn action() -> Final {
        return final {
            hits = hits.unwrap_or(0u64) + 1u64;
        };
    }

    @noupgrade
    constructor() {}
}
