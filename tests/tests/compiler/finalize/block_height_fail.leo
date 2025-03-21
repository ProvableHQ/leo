
program test.aleo {
    transition matches(height: u32) {
        assert_eq(height, block.height);
        return;
    }
}
