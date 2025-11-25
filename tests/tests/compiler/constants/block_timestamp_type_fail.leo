
program test.aleo {
    async function bar() {
        let x: u32 = block.timestamp;
    }

    async transition foo() -> Future {
        return bar();
    }
}
