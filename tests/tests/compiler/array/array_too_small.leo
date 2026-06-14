program test.aleo {
    fn foo(a: [bool; 0]) -> bool {
        let _bar = [false; 0];
        return true;
    }

    @noupgrade
    constructor() {}
}
