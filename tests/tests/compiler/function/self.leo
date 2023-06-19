/*
namespace: Compile
expectation: Pass
*/

program test.aleo {
    transition matches(addr: address) -> bool {
        return self.caller == addr;
    }
}
