program test.aleo {
    fn foo() {}

    @noupgrade
    constructor() {
        assert_eq(self.edition, 0u16);
    }
}
