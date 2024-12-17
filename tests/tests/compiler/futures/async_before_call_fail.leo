/*
namespace = "Compile"
expectation = "Fail"
*/

program child.aleo {
    transition t() {}
}

 // --- Next Program --- //

import child.aleo;

program parent.aleo {
    async transition foo() -> Future {
        let future: Future = finalize_foo();
        child.aleo/t();
        return future;
    }

    async function finalize_foo() {}
}
