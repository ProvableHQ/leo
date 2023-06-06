/*
namespace: Compile
expectation: Fail
*/

program test.aleo {
    transition main(
        group_value: group,
    ) -> group {
        let a: group = group::GENERATOR;

        return group_value + a;
    }
}
