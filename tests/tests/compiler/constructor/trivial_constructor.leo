program test.aleo {
    transition foo() {}

    @custom
    async constructor() {
        assert_eq(true, true);
    }
}
