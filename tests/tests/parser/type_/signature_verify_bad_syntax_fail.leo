/*
namespace: Parse
expectation: Fail
*/

program test.aleo {
    transition baz(s: signature, a: address, v: value) {
        let a: bool = signature.verify(s, a, v);
        assert(a, b);
    }

   transition bar(s: signature, a: address, v: value) {
        let b: bool = s.verify(a, a, v);
        assert_eq(a, b);
    }

    transition foo(signature: field) -> u8 {
        return 0u8;
    }

    transition signature(foo: field) -> u8 {
        return 0u8;
    }
}
