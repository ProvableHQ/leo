struct Data {
    amount: u64,
}

program test.aleo {
    @noupgrade
    constructor() {}

    mapping store: field => Data;

    fn save(d: Data) {
        return;
    }
}
