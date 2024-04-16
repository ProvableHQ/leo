/*
namespace: Compile
expectation: Pass
*/

program test.aleo {
    mapping data: address => [bool; 8];

    async transition foo(a: [bool; 8]) -> Future {
        return fin_foo(self.caller, a);
    }

    async function fin_foo(caller: address, a: [bool; 8]) {
        data.set(caller, a);
    }
}
