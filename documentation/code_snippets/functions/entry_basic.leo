program hello.aleo {
    fn foo(
        public a: field,
        b: field,
    ) -> field {
        return a + b;
    }

    @noupgrade
    constructor() {}
}
