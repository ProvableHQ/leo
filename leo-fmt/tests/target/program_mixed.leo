program test.aleo {
    struct Data {
        value: u64,
    }

    mapping store: field => Data;

    transition save(d: Data) {
        return;
    }
}
