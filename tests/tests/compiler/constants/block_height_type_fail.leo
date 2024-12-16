/*
 namespace = "Compile"
 expectation = "Fail"
 */

program test.aleo {
    async function bar() {
        let x: u64 = block.height;
    }

    async transition foo() -> Future {
        return bar();
    }
}
