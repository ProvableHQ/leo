
program test.aleo {
    fn matches(height: u32) {
        assert_eq(height, block.height);
        return;
    }
}
