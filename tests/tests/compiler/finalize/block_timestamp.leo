program test.aleo {
    fn matches(timestamp: i64) -> Final {
        return final { finalize_matches(timestamp); };
    }

    @noupgrade
    constructor() {}
}

export final fn finalize_matches(timestamp: i64) {
    assert_eq(timestamp, block.timestamp);
}
