// ANCHOR: file
struct foo {
    a: u8,
    b: scalar
}

program test.aleo {
    fn verify_field(s: signature, a: address, v: field) {
        let first: bool = signature::verify(s, a, v);
        let second: bool = s.verify(a, v);
        assert_eq(first, second);
    }

    fn verify_foo(s: signature, a: address, v: foo) {
        let first: bool = signature::verify(s, a, v);
        let second: bool = s.verify(a, v);
        assert_eq(first, second);
    }

    @noupgrade
    constructor() {}
}
// ANCHOR_END: file
