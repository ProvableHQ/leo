/*
namespace: Compile
expectation: Pass
*/
program test.aleo {
    async transition main(a: u16) -> Future {
        return finalize_main(a);
    }

    async function finalize_main(a: u16) {
        assert_eq(1u16, network.id);
        assert_eq(a, network.id);
        assert_eq(network.id, network.id);
    }
}
