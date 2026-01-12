
program test.aleo {
    fn matches(height: u32) -> Final {
        return final { finalize_matches(height); };
    }
}

final fn finalize_matches(height: u32) {
    assert_eq(height, block.height);
}
