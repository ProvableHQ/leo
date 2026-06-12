interface MyInterface {
    fn do_something() -> u64;
}

program lib.aleo {
    fn unused() {}

    @noupgrade
    constructor() {}
}

// --- Next Program --- //
import lib.aleo;

program test.aleo: lib.aleo::MyInterface {
    fn do_something() -> u64 {
        return 1u64;
    }

    @noupgrade
    constructor() {}
}
