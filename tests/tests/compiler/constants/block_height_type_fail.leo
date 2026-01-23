onchain fn bar() {
    let x: u64 = block.height;
}

program test.aleo {
    fn foo() -> Future {
        return bar();
    }
}
