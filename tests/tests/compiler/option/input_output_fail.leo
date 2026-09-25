// --- library: optional_lib --- //

export struct Wrapper {
    x: u8?,
}

// --- Next Program --- //
export struct Bar {
    x: u8?,
}

export struct Foo {
    b: Bar,
}

program test.aleo {
    fn foo(x: u8?, y: [u8?; 3], f: Foo) -> (u8?, u8?, Foo) {
        return (x, x, f);
    }

    fn bar(x: u8?, f: Foo) {
        return final {};
    }

    view fn view_optional(x: u8?, y: [u8?; 3], f: Foo) -> (u8?, [u8?; 3], Foo) {
        return (x, y, f);
    }

    view fn library_optional(x: Wrapper) -> Wrapper {
        return x;
    }

    @noupgrade
    constructor() {}
}

@no_inline
export fn baz(x: u8?) -> u8? {
    return x;
}

export struct Wrapper {
    inner: optional_lib::Wrapper,
}
