program test.aleo {
    record Foo::[N: u32] {
        owner: address,
        arr: [u8; N],
    }

    record Bar::[N: field] {
        owner: address
    }

    fn foo() {}
}
