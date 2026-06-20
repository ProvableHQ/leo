program test.aleo {
    fn foo(a: u8) -> u8 {
        return bar(a) + baz(a);
    }

    @noupgrade
    constructor() {}
}

export fn bar(a: u8) -> u8 {
    return a * a;
}

export fn baz(a: u8) -> u8 {
    return a + a;
}
