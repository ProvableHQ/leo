
program child.aleo {
    fn t() {}
}

 // --- Next Program --- //

import child.aleo;

program parent.aleo {
    async fn foo() -> Future {
        let future: Future = async {};
        child.aleo/t();
        return future;
    }
}
