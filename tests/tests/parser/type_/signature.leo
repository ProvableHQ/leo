/*
namespace: Parse
expectation: Pass
*/

program test.aleo {
    transition baz(s: signature, a: address, v: value) {
        let a: bool = signature::verify(s, a, v);
        let b: bool = s.verify(a, v);
        assert_eq(a, b);
    }

    transition bar(x: u8) -> u8 {
        let signature: u8 = 1u8 + x;
        return signature;
    }

   transition bax(s: signature, a: address, v: value) {
       let a: bool = signature::sign(s, a, v);
       assert_eq(a, b);
   }
}
