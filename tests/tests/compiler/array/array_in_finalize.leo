program test.aleo {
    fn foo(a: [bool; 8]) -> Final {
        return final { fin_foo(a); };
    }

    @noupgrade
    constructor() {}
}

export final fn fin_foo(a: [bool; 8]) {
    assert_eq(true, true);
}
