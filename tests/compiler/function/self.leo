/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    transition matches(addr: address) -> bool {
        async finalize(self.caller);
        return self.caller == addr;
    } finalize matches(addr: address) -> bool {
        return addr == self.caller;
    }
}
