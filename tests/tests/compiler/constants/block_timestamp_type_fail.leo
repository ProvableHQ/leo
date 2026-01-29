
onchain fn bar() {
    let x: u32 = block.timestamp;
}

program test.aleo {
    fn foo() -> Final {
        return bar();
    }
}
