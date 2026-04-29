program example.aleo {
    storage history: [u64];

    fn append(value: u64) -> Final {
        return final {
            history.push(value);
        };
    }

    @noupgrade
    constructor() {}
}
