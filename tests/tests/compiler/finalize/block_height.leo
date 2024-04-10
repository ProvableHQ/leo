/*
namespace: Compile
expectation: Pass
*/

program test.aleo {
    async transition matches(height: u32) -> Future {
        return finalize_matches(height);
    }

    async function finalize_matches(height: u32) {
        assert_eq(height, block.height);
    }
}
