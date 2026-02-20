// TODO: Make this test pass. For now, we error out on consts that have optional types in them.

// === CONSTS ===

const A: u8? = 10u8;
const B: [u8?; 3] = [1u8, 2u8, 3u8];
const C: (u8, u8?) = (42u8, 99u8);
const D: [[u8?; 2]; 2] = [[4u8, 5u8], [6u8, 7u8]];

struct MyStruct {
    x: u8?,
}

const E: MyStruct = MyStruct { x: 88u8 };
const F: [MyStruct; 2] = [MyStruct { x: 11u8 }, MyStruct { x: 22u8 }];

program const_optionals.aleo {
    // === TRANSITIONS ===

    fn const_single_optional() -> u8 {
        return A.unwrap();
    }

    fn const_array_of_optionals() -> u8 {
        return B[1].unwrap();
    }

    fn const_tuple_with_optional() -> u8 {
        return C.1.unwrap();
    }

    fn const_nested_array() -> u8 {
        const D_local: [[u8?; 2]; 2] = [[1u8, 2u8], [3u8, 4u8]];
        return D_local[0][1].unwrap() + D[0][1].unwrap();
    }

    fn const_struct_with_optional() -> u8 {
        return E.x.unwrap();
    }

    fn const_array_of_structs() -> u8 {
        return F[0].x.unwrap() + F[1].x.unwrap();
    }

    @noupgrade
    constructor() {}
}
