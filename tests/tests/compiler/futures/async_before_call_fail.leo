
program child.aleo {
    fn t() {}
}

 // --- Next Program --- //

import child.aleo;

program parent.aleo {
    fn foo() -> Final {
        let future: Final = finalize_foo();
        child.aleo/t();
        return future;
    }
}

final fn finalize_foo() {}
