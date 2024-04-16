/*
namespace: Compile
expectation: Pass
*/

program test.aleo {
    async transition foo(a: [bool; 8]) -> Future {
        return fin_foo(a);
    }

    async function fin_foo(a: [bool; 8]) {
        assert_eq(true, true);
    }
}
