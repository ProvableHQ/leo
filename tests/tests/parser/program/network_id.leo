/*
namespace: Parse
expectation: Pass
*/
program test.aleo {
    function main(a: address) {
        assert_eq(network.id, 1u32);
        return a;
    }
}
