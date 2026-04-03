
program test.aleo {
    storage counter: u8;

    fn bar() -> Final {
        return final {
            baz();
        };
    }

    fn baz() -> Final {
        return final {
            counter = 1u8;
        };
    }

    @noupgrade
    constructor() {}
}
