/*
namespace: Compile
expectation: Fail
*/

program test.aleo {

    struct foo {
        a: u8,
        b: scalar
    }

    transition verify_field(s: signature, a: address, v: field) {
        let first: bool = signature::verify(a, s, v);
        let second: bool = s.verify(1u8, v);
        assert_eq(first, second);
    }

    transition verify_foo(s: signature, a: address, v: foo) {
        let first: bool = signature::verify(a, v, s);
        let second: bool = s.verify(v, a);
        assert_eq(first, second);
    }
}
