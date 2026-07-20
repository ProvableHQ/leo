program test.aleo {
    fn foo() {}

    @noupgrade
    constructor() {
        assert_eq(std::ctx::edition(), 0u16);
    }
}
