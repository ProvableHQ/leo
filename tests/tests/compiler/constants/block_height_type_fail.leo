async fn bar() {
    let x: u64 = block.height;
}

program test.aleo {
    async fn foo() -> Future {
        return bar();
    }
}
