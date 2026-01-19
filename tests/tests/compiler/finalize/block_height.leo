
program test.aleo {
    async fn matches(height: u32) -> Future {
        return finalize_matches(height);
    }
}

async fn finalize_matches(height: u32) {
    assert_eq(height, block.height);
}
