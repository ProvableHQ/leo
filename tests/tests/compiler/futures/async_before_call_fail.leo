program child.aleo {
    fn t() {}

    @noupgrade
    constructor() {}
}

// --- Next Program --- //
import child.aleo;

program parent.aleo {
    fn foo() -> Final {
        let future: Final = final { finalize_foo(); };
        child.aleo::t();
        return future;
    }

    @noupgrade
    constructor() {}
}

final fn finalize_foo() {}
