final fn bar() {
    let x: u64 = block.height;
}

program test.aleo {
    fn foo() -> Final {
        return final { bar(); };
    }
}
