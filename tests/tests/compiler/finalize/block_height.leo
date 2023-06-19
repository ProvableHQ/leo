/*
namespace: Compile
expectation: Pass
*/

program test.aleo {
    transition matches(height: u32) {
        return then finalize(self.caller);
    } finalize matches(height: u32) {
        assert_eq(height, block.height);
    }
}
