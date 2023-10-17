/*
namespace: Compile
expectation: Pass
*/

program test.aleo {
    mapping data: address => [bool; 8];

    transition foo(a: [bool; 8]) {
        return then finalize(self.caller, a);
    }

    finalize foo(caller: address, a: [bool; 8]) {
        data.set(caller, a);
    }
}
