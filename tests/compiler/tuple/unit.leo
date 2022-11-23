/*
namespace: Compile
expectation: Pass
*/

program test.aleo {

    transition foo(a: u8, b: u8) -> () {
        console.assert_eq(a, b);
        console.assert_eq(b, a);
        return ();
    }

    transition bar(a: u8, b: u8) -> () {
        console.assert_eq(a, b);
        console.assert_eq(b, a);
        return;
    }

    transition baz(a: u8, b: u8) -> () {
        console.assert_eq(a, b);
        console.assert_eq(b, a);
    }

    transition floo(a: u8, b: u8) {
        console.assert_eq(a, b);
        console.assert_eq(b, a);
        return ();
    }

    transition blar(a: u8, b: u8) {
        console.assert_eq(a, b);
        console.assert_eq(b, a);
        return;
    }

    transition blaz(a: u8, b: u8) {
        console.assert_eq(a, b);
        console.assert_eq(b, a);
    }
}
