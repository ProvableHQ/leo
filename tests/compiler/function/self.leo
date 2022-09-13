/*
namespace: Compile
expectation: Pass
*/

@program
function matches(addr: address) -> bool {
    finalize(self.caller);
    return self.caller == addr;
} finalize matches(addr: address) -> bool {
    return addr == self.caller;
}
