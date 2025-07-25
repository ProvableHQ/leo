
program child.aleo {
    transition t() {}
}

 // --- Next Program --- //

import child.aleo;

program parent.aleo {
    async transition foo() -> Future {
        let future: Future = async {};
        child.aleo/t();
        return future;
    }
}
