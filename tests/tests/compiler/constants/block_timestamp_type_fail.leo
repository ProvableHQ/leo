
async fn bar() {
    let x: u32 = block.timestamp;
}

program test.aleo {
    async fn foo() -> Future {
        return bar();
    }
}
