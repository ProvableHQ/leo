// Compilable scaffold for `documentation/language/03_data_types.md`. Each
// anchored region maps to one inline snippet in the doc.

struct Bar {
    data: [u8; 8],
}

struct S {
    x: field,
    y: u32,
}

struct Matrix::[N: u32, M: u32] {
    data: [field; N * M],
}

struct Point {
    x: u32?,
    y: u32?,
}

struct InferFoo {
    x: u8,
}

struct foo {
    a: u8,
    b: scalar,
}

program data_types_demo.aleo {

    // ANCHOR: token_record
    record Token {
        // The token owner.
        owner: address,
        // The token amount.
        amount: u64,
    }
    // ANCHOR_END: token_record

    // ANCHOR: array_mapping
    // Declare a mapping that contains array values
    mapping data: address => [bool; 8];
    // ANCHOR_END: array_mapping

    fn demo() -> u32 {
        // ANCHOR: addresses
        let sender: address = aleo1ezamst4pjgj9zfxqq0fwfj8a4cjuqndmasgata3hggzqygggnyfq6kmyd4;
        let receiver = aleo129nrpl0dxh4evdsan3f4lyhz5pdgp6klrn5atp37ejlavswx5czsk0j5dj;
        // ANCHOR_END: addresses

        // ANCHOR: booleans
        let b: bool = false;
        let a = false;
        // ANCHOR_END: booleans

        // ANCHOR: int_basic
        let bint: u8 = 1u8;
        // ANCHOR_END: int_basic

        // ANCHOR: int_underscore
        let n: u64 = 1_000_000u64;
        // ANCHOR_END: int_underscore

        // ANCHOR: int_cast
        let ac: u8 = 2u8; // explicit type
        let bc: u16 = ac as u16; // type casting
        // ANCHOR_END: int_cast

        // ANCHOR: field_literals
        let af: field = 0field;
        let bf = 8444461749428370424248824938781546531375899335154063827935233455917409239040field;
        let cf: field = 0;
        // ANCHOR_END: field_literals

        // ANCHOR: group_literals
        let ag: group = 0group; // the point with 0 x-coordinate, (0, 1)
        let bg = 1540945439182663264862696551825005342995406165131907382295858612069623286213group;  // the generator point
        let cg: group = 0;
        // ANCHOR_END: group_literals

        // ANCHOR: group_gen
        let g: group = group::GEN; // the group generator
        // ANCHOR_END: group_gen

        // ANCHOR: scalar_literals
        let asc: scalar = 0scalar;
        let bsc = 2111115437357092606062206234695386632838870926408408195193685246394721360382scalar;
        let csc: scalar = 0;
        // ANCHOR_END: scalar_literals

        // ANCHOR: signature_literal
        let sig: signature = sign195m229jvzr0wmnshj6f8gwplhkrkhjumgjmad553r997u7pjfgpfz4j2w0c9lp53mcqqdsmut2g3a2zuvgst85w38hv273mwjec3sqjsv9w6uglcy58gjh7x3l55z68zsf24kx7a73ctp8x8klhuw7l2p4s3aq8um5jp304js7qcnwdqj56q5r5088tyvxsgektun0rnmvtsuxpe6sj;
        // ANCHOR_END: signature_literal

        // ANCHOR: identifier_literal
        let target: identifier = 'my_program';
        // ANCHOR_END: identifier_literal

        // ANCHOR: arrays_basic
        // Initialize a boolean array of length 4
        let arr: [bool; 4] = [true, false, true, false];

        // Empty array
        let empty: [u32; 0] = [];

        // Nested array
        let nested: [[bool; 2]; 2] = [[true, false], [true, false]];
        // ANCHOR_END: arrays_basic

        // ANCHOR: array_of_structs
        // Array of structs
        let arr_of_structs: [Bar; 2] = [
            Bar { data: [1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8] },
            Bar { data: [2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8] },
        ];
        // ANCHOR_END: array_of_structs

        // ANCHOR: tuple_basic
        // Initialize a tuple of mixed types
        let tup: (u8, u8, bool) = (1u8, 1u8, true);

        // Nested tuple
        let nestedt: [[bool; 2]; 2] = [[true, false], [true, false]];
        // ANCHOR_END: tuple_basic

        // ANCHOR: const_generic_struct
        let m = Matrix::[2u32, 2u32] { data: [0field, 1field, 2field, 3field] };
        // ANCHOR_END: const_generic_struct

        // ANCHOR: option_basic
        let w: u8? = 42u8;
        let x: u8? = none;
        // ANCHOR_END: option_basic

        // ANCHOR: option_unwrap
        let yo = w.unwrap();        // Returns 42u8
        let zo = x.unwrap_or(99u8); // Returns 99u8
        // ANCHOR_END: option_unwrap

        // ANCHOR: option_composite
        // Array of options
        let arro: [u16?; 2] = [1u16, none];
        let first_val = arro[0].unwrap();        // Returns 1u16
        let second_val = arro[1].unwrap_or(0u16); // Returns 0u16

        // Structs have option variant as well
        let p: Point? = none;
        let p_val = p.unwrap_or(Point { x: 0u32, y: none }); // Returns default
        // ANCHOR_END: option_composite

        // ANCHOR: type_inference
        let ainf: u8 = 2u8; // explicit type - allowed
        let binf = 2u8; // type inference - allowed
        let cinf: u8 = 2; // type inference - allowed
        // ANCHOR_END: type_inference

        // ANCHOR: type_inference_struct
        let f = InferFoo {
            x: 5, // inferred to be a `u8`
        };
        // ANCHOR_END: type_inference_struct

        return 0u32;
    }

    // ANCHOR: array_access_fn
    fn array_access(a: [Bar; 8]) -> [u8; 8] {
        return a[0u8].data;
    }
    // ANCHOR_END: array_access_fn

    // ANCHOR: sum_with_loop_fn
    fn sum_with_loop(a: [u64; 4]) -> u64 {
        let sum: u64 = 0u64;
        for i: u8 in 0u8..4u8 {
            sum += a[i];
        }
        return sum;
    }
    // ANCHOR_END: sum_with_loop_fn

    // ANCHOR: tuple_access_fn
    fn tuple_access(fa: u8, ba: u8) -> u8 {
        let a: (u8, u8) = (fa, ba);
        let result: u8 = a.0 + a.1;
        return result;
    }
    // ANCHOR_END: tuple_access_fn

    // ANCHOR: signature_verify
    fn verify_field(s: signature, a: address, v: field) {
        let first: bool = signature::verify(s, a, v);
        let second: bool = s.verify(a, v);
        assert_eq(first, second);
    }

    fn verify_foo(s: signature, a: address, v: foo) {
        let first: bool = signature::verify(s, a, v);
        let second: bool = s.verify(a, v);
        assert_eq(first, second);
    }
    // ANCHOR_END: signature_verify

    // ANCHOR: struct_construct
    fn make_s(y: u32) -> S {
        let s: S = S {
            x: 172field,
            y,
        };
        return s;
    }
    // ANCHOR_END: struct_construct

    @noupgrade
    constructor() {}
}
