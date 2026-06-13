program test.aleo {
    fn foo(a: [bool; 8]) -> bool {
        return true;
    }

    fn bar(a: [bool; 8]) -> [bool; 8] {
        return a;
    }

    @noupgrade
    constructor() {}
}

fn _baz(a: [bool; 8]) -> bool {
    assert(a[0u8]);
    return true;
}

fn _qux(a: [bool; 8]) -> [bool; 8] {
    assert(a[0u8]);
    return a;
}
