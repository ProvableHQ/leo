program test.aleo {
    fn foo(private a: u8) -> Final {
        return bar();
    }

    @noupgrade
    constructor() {}
}

final fn bar() {}
