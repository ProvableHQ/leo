/*
namespace: Compile
expectation: Fail
*/

program test.aleo {    
    async transition matches(addr: address) -> (bool, Future) {
        return (self.caller == addr, finalize_matches(self.caller));
    }
    async function finalize_matches(addr: address) {
        assert_eq(addr, self.caller);
    }
}
