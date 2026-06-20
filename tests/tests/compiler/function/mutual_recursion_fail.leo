@no_inline
export fn foo(n: u8) -> u8 {
    return bar(n);
}

@no_inline
export fn bar(n: u8) -> u8 {
    return foo(n);
}

program test.aleo {
    fn baz(n: u8) -> u8 {
        return bax(n);
    }

    fn bax(n: u8) -> u8 {
        return baz(n);
    }

    @noupgrade
    constructor() {}
}
