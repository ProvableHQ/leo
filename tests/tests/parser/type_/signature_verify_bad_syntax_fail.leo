
program test.aleo {
    fn baz(s: signature, a: address, v: value) {
        let a: bool = signature.verify(s, a, v);
        assert(a, b);
    }

   fn bar(s: signature, a: address, v: value) {
        let b: bool = s.verify(a, a, v);
        assert_eq(a, b);
    }

    fn foo(signature: field) -> u8 {
        return 0u8;
    }

    fn signature(foo: field) -> u8 {
        return 0u8;
    }
}
