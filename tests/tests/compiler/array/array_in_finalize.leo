/*
namespace: Compile
expectation: Pass
*/

program test.aleo {
    transition foo(a: [bool; 8]) {
        return then finalize a;
    }

    finalize foo(a: [bool; 8]) {
        assert_eq(true, true);
    }
}
