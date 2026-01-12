
async function bar() {
    let x: u32 = block.timestamp;
}

program test.aleo {
    async transition foo() -> Future {
        return bar();
    }
}
