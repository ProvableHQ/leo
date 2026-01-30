
struct bar {
    data: u8,
}

program test.aleo {
    fn foo(a: [bar; 8]) -> u8 {
        return a[0u8].data;
    }
}
