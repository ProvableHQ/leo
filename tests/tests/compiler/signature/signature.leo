/*
namespace: Compile
expectation: Pass
*/

program test.aleo {

    struct foo {
        a: u8,
        b: scalar
    }

    transition verify_field(s: signature, a: address, v: field) {
        let first: bool = signature::verify(s, a, v);
        let second: bool = s.verify(a, v);
        assert_eq(first, second);
    }

    transition verify_foo(s: signature, a: address, v: foo) {
        let first: bool = signature::verify(s, a, v);
        let second: bool = s.verify(a, v);
        assert_eq(first, second);
    }

    transition literal_ops_on_signature(a: bool, b: signature, c: signature) -> signature {
        let equal: bool = b == c;
        let not_equal: bool = b != c;
        let ternary: signature = a ? b : c;

        return ternary;
    }
}
