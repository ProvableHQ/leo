/*
namespace: Compile
expectation: Pass
*/

program test.aleo {
    transition main(a: group) -> field {
        return group::to_y_coordinate(a);
    }
}
