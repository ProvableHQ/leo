
program test.aleo {
    fn matches(height: u32) -> Future {
        return finalize_matches(height);
    }
}

onchain fn finalize_matches(height: u32) {
    assert_eq(height, block.height);
}
