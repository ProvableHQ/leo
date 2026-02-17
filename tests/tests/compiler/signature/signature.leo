struct foo {
    a: u8,
    b: scalar
}

const S: signature = sign195m229jvzr0wmnshj6f8gwplhkrkhjumgjmad553r997u7pjfgpfz4j2w0c9lp53mcqqdsmut2g3a2zuvgst85w38hv273mwjec3sqjsv9w6uglcy58gjh7x3l55z68zsf24kx7a73ctp8x8klhuw7l2p4s3aq8um5jp304js7qcnwdqj56q5r5088tyvxsgektun0rnmvtsuxpe6sj;

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

    fn literal_ops_on_signature(a: bool, b: signature, c: signature) -> signature {
        let equal: bool = b == c;
        let not_equal: bool = b != c;
        let ternary: signature = a ? b : c;

        return ternary;
    }

    fn literal_test(s0: signature) -> (bool, signature, signature) {
        let s1: signature = sign195m229jvzr0wmnshj6f8gwplhkrkhjumgjmad553r997u7pjfgpfz4j2w0c9lp53mcqqdsmut2g3a2zuvgst85w38hv273mwjec3sqjsv9w6uglcy58gjh7x3l55z68zsf24kx7a73ctp8x8klhuw7l2p4s3aq8um5jp304js7qcnwdqj56q5r5088tyvxsgektun0rnmvtsuxpe6sj;
      
        assert(s0 == s1);

        let same = sign195m229jvzr0wmnshj6f8gwplhkrkhjumgjmad553r997u7pjfgpfz4j2w0c9lp53mcqqdsmut2g3a2zuvgst85w38hv273mwjec3sqjsv9w6uglcy58gjh7x3l55z68zsf24kx7a73ctp8x8klhuw7l2p4s3aq8um5jp304js7qcnwdqj56q5r5088tyvxsgektun0rnmvtsuxpe6sj
                      != sign195m229jvzr0wmnshj6f8gwplhkrkhjumgjmad553r997u7pjfgpfz4j2w0c9lp53mcqqdsmut2g3a2zuvgst85w38hv273mwjec3sqjsv9w6uglcy58gjh7x3l55z68zsf24kx7a73ctp8x8klhuw7l2p4s3aq8um5jp304js7qcnwdqj56q5r5088tyvxsgektun0rnmvtsuxpe6sj;

        return (same, s1, S);
    }
}
