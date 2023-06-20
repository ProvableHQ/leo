/*
namespace: Compile
expectation: Pass
*/

program test.aleo {
    transition matches(height: u32) {
        return then finalize(height);
    } finalize matches(height: u32) {
        assert_eq(height, block.height);
    }
}
