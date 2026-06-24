program test.aleo {
    fn matches(height: u32) {
        assert_eq(height, std::ctx::block_height());
        return;
    }

    @noupgrade
    constructor() {}
}
