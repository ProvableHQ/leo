program test.aleo {
    transition foo() {}

    @noupgrade
    async constructor() {
        assert_eq(self.edition, 0u16);
    }
}
