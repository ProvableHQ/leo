export interface Foo {
    fn foo();
}

program bar.aleo {
    fn foo() {}

    fn bar() {}

    @noupgrade
    constructor() {}
}
