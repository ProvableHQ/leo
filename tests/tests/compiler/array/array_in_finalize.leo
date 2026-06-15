program test.aleo {
    fn foo(a: [bool; 8]) -> Final {
        return final { fin_foo(a); };
    }

    @noupgrade
    constructor() {}
}

final fn fin_foo(_a: [bool; 8]) {
    assert_eq(true, true);
}
