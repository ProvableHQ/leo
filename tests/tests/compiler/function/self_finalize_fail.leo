/*
namespace: Compile
expectation: Fail
*/

program test.aleo {    
    transition matches(addr: address) -> bool {
        return self.caller == addr then finalize(self.caller);
    } finalize matches(addr: address) {
        assert_eq(addr, self.caller);
    }
}
