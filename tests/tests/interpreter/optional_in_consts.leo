/*
seed = 2468101214

[case]
program = "const_optionals.aleo"
function = "const_single_optional"
input = []

[case]
program = "const_optionals.aleo"
function = "const_array_of_optionals"
input = []

[case]
program = "const_optionals.aleo"
function = "const_tuple_with_optional"
input = []

[case]
program = "const_optionals.aleo"
function = "const_nested_array"
input = []

[case]
program = "const_optionals.aleo"
function = "const_struct_with_optional"
input = []

[case]
program = "const_optionals.aleo"
function = "const_array_of_structs"
input = []

[case]
program = "const_optionals.aleo"
function = "inline_const_optional"
input = []

[case]
program = "const_optionals.aleo"
function = "inline_const_array_optional"
input = []

[case]
program = "const_optionals.aleo"
function = "inline_const_struct_optional"
input = []
*/

program const_optionals.aleo {

    // === Top-Level Constants ===
    const X: u8? = 10u8;
    const Y: [u8?; 3] = [1u8, 2u8, 3u8];
    const T: (u8, u8?) = (99u8, 100u8);
    const ARR_NESTED: [[u8?; 2]; 2] = [[5u8, 6u8], [7u8, 8u8]];
    const S: Foo = Foo { x: 88u8 };
    const STRUCT_ARR: [Foo?; 2] = [Foo { x: 33u8 }, Foo { x: 34u8 }];

    struct Foo {
        x: u8?,
    }

    // === Transitions ===

    // Top-level const unwrap
    transition const_single_optional() -> u8 {
        return X.unwrap();
    }

    // Top-level const array index + unwrap
    transition const_array_of_optionals() -> u8 {
        return Y[1].unwrap(); // 2u8
    }

    // Top-level tuple with optional element
    transition const_tuple_with_optional() -> u8 {
        return T.1.unwrap(); // 100u8
    }

    // Nested array access
    transition const_nested_array() -> u8 {
        return ARR_NESTED[0][0].unwrap(); // 5u8
    }

    // Struct with optional field
    transition const_struct_with_optional() -> u8 {
        return S.x.unwrap(); // 88u8
    }

    // Array of optional structs
    transition const_array_of_structs() -> u8 {
        return STRUCT_ARR[0].unwrap().x.unwrap(); // 33u8
    }

    // Inline const (optional value)
    transition inline_const_optional() -> u8 {
        const A: u8? = 55u8;
        return A.unwrap();
    }

    // Inline const (optional in array)
    transition inline_const_array_optional() -> u8 {
        const B: [u8?; 2] = [9u8, 10u8];
        return B[1].unwrap();
    }

    // Inline const (struct with optional)
    transition inline_const_struct_optional() -> u8 {
        const C: Foo = Foo { x: 77u8 };
        return C.x.unwrap();
    }

    @noupgrade
    async constructor() {}
}
