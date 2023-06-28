/*
namespace: Compile
expectation: Pass
*/

program test.aleo {
    transition main(a: group) -> field {
        let x: field = a.to_x_coordinate();
        let y: field = 0group.to_y_coordinate();
        return group::to_x_coordinate(a);
    }
}
