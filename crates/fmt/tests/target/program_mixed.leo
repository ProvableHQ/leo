struct Data {
    value: u64,
}
program test.aleo {
    mapping store: field => Data;

    fn save(d: Data) {
        return;
    }
}
