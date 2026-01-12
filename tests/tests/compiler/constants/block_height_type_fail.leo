async function bar() {
    let x: u64 = block.height;
}

program test.aleo {
    async transition foo() -> Future {
        return bar();
    }
}
