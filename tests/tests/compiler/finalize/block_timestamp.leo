
program test.aleo {
    async fn matches(timestamp: i64) -> Future {
        return finalize_matches(timestamp);
    }
}

async fn finalize_matches(timestamp: i64) {
    assert_eq(timestamp, block.timestamp);
}
