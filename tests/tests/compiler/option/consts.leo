program const_optionals.aleo {
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

    // === TRANSITIONS ===

    transition const_single_optional() -> u8 {
        return A.unwrap(); // 10
    }

    transition const_array_of_optionals() -> u8 {
        return B[1].unwrap(); // 2
    }

    transition const_tuple_with_optional() -> u8 {
        return C.1.unwrap(); // 99
    }

    transition const_nested_array() -> u8 {
        return D[0][1].unwrap(); // 5
    }

    transition const_struct_with_optional() -> u8 {
        return E.x.unwrap(); // 88
    }

    transition const_array_of_structs() -> u8 {
        return F[0].x.unwrap() + F[1].x.unwrap(); // 11 + 22 = 33
    }

    @noupgrade
    async constructor() {}
}
