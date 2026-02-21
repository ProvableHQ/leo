
program test.aleo {
    fn baz(s: signature, a: address, v: value) {
        let a: bool = signature::verify(s, a, v);
        let b: bool = s.verify(a, v);
        assert_eq(a, b);
    }

    fn bar(x: u8) -> u8 {
        let signature_: u8 = 1u8 + x;
        return signature_;
    }
}
