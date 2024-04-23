/*
namespace: Compile
expectation: Pass
*/
program test.aleo {
    transition main(a: address) -> address {
        assert_eq(network.id, network.id);
        return a;
    }
}
