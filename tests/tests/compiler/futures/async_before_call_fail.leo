
program child.aleo {
    fn t() {}
}

 // --- Next Program --- //

import child.aleo;

program parent.aleo {
    fn foo() -> Future {
        let future: Future = finalize_foo();
        child.aleo/t();
        return future;
    }
}

onchain fn finalize_foo() {}
