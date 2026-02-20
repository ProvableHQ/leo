
program test.aleo {
    record bar {
        owner: address,
        data: u8,
    }

    fn foo(a: [bar; 8]) -> u8 {
        return a[0u32].data;
    }
}
