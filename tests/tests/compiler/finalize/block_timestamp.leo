
program test.aleo {
    async transition matches(timestamp: i64) -> Future {
        return finalize_matches(timestamp);
    }

    async function finalize_matches(timestamp: i64) {
        assert_eq(timestamp, block.timestamp);
    }
}
