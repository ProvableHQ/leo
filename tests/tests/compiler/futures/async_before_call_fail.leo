
program child.aleo {
    fn t() {}
}

 // --- Next Program --- //

import child.aleo;

program parent.aleo {
    async fn foo() -> Future {
        let future: Future = finalize_foo();
        child.aleo/t();
        return future;
    }
}

async fn finalize_foo() {}
