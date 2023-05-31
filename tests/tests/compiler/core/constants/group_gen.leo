/*
namespace: Compile
expectation: Pass
*/

program test.aleo {
    transition main(
        group_value: group,
    ) -> group {
        let a: group = group::GEN;

        return group_value + a;
    }
}
