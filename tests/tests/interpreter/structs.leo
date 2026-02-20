/*
seed = 123456789

//[case]
//program = "test.aleo"
//function = "makeone"
//input = ["1field", "2u32"]

[case]
program = "test.aleo"
function = "makeconsts"
input = ["1field", "2field"]
*/

struct SomeStruct {
    x: field,
    y: u32,
}

struct StructConsts::[X: u32, Y: u32] {
    xs: [field; X + Y],
}

program test.aleo {
    fn makeone(x: field, y: u32) -> SomeStruct {
        return SomeStruct {
            x,
            y,
        };
    }

    fn makeconsts(x: field, y: field) -> StructConsts::[1u32, 2u32] {
        return StructConsts::[1u32, 2u32] {
            xs: [x, y, y],
        };
    }

    @noupgrade
    constructor() {}
}
