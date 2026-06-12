program test.aleo {
    @noupgrade
    constructor() {}

    mapping allowances: [address; 2] => u128;

    fn foo() {}
}
