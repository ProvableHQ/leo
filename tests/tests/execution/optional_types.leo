program all_types.aleo {

    // --- Shared Structs ---
    struct OptStruct {
        val: u16?,
    }

    struct MyStruct {
        x: u8,
    }

    struct Foo {
        x: u8,
    }

    struct Inner {
        val: u8?,
    }

    struct Wrapper {
        arr: [Inner?; 2]?,
    }

    struct Container {
        wrappers: [Wrapper?; 2]?,
    }

    struct Top {
        container: Container?,
    }

    // --- Section 1: Basic Optionals and Inference ---
    transition optional_basic_primitives() -> u32 {
        let a: u32? = 10;
        let b: u32? = none;
        let c: u32 = a.unwrap();
        let d: u32 = b.unwrap_or(42);
        return c + d; // 10 + 42 = 52
    }

    // --- Section 2: Arrays of Optionals ---
    transition array_of_optionals() -> u8 {
        let arr: [u8?; 3] = [1, none, 3];
        let sum = arr[0].unwrap_or(10)
                  + arr[1].unwrap_or(20)
                  + arr[2].unwrap_or(30);
        return sum; // 1 + 20 + 3 = 24
    }

    // --- Section 3: Optional Arrays ---
    transition optional_array() -> u128 {
        let arr_opt: [u128?; 2]? = [1000, none];
        let arr_opt2: [u128?; 3]? = [42; 3];
        let first =   arr_opt.unwrap()[0].unwrap_or(10) 
                    + arr_opt.unwrap()[1].unwrap_or(11) 
                    + arr_opt2.unwrap()[0].unwrap_or(12)
                    + arr_opt2.unwrap()[2].unwrap_or(13);
        return first;
    }

    // --- Section 4: Structs with Optional Fields ---
    transition optional_struct_field() -> u16 {
        let s = OptStruct { val: none };
        return s.val.unwrap_or(123); // 123
    }

    // --- Section 5: Optional Structs ---
    transition optional_struct() -> u8 {
        let s: MyStruct? = MyStruct { x: 5 };
        return s.unwrap().x; // 5
    }

    // --- Section 6: Arrays of Optional Structs ---
    transition array_of_optional_structs() -> u8 {
        let arr: [Foo?; 2] = [Foo { x: 10 }, none];
        return arr[0].unwrap().x; // 10
    }

    // --- Section 7: Nested Optionals in Structs & Arrays ---
    transition nested_optional_structs() -> u8 {
        let inner = Inner { val: 10 };
        let wrapper = Wrapper { arr: [inner, none] };
        let container = Container { wrappers: [wrapper, none] };
        let top = Top { container };

        return top.container.unwrap()
            .wrappers.unwrap()[0].unwrap()
            .arr.unwrap()[0].unwrap()
            .val.unwrap(); // 10
    }

    // --- Section 8: Tuples with Optional Elements ---
    transition tuples_with_optional_elements() -> u32 {
        let t1: (u32, u32?) = (100, 200);
        let t2: (u32, u32?) = (300, none);

        let sum = t1.0 + t1.1.unwrap_or(0)
                + t2.0 + t2.1.unwrap_or(50);
        return sum; // 100 + 200 + 300 + 50 = 650
    }

    // --- Section 9: Optionals with address, group, field, scalar, signature ---
    transition optional_misc_primitive_types() -> (group, field, scalar) {
        let gr: group? = 0;
        let fl: field? = 1;
        let sc: scalar? = 2;
        
        return (
            gr.unwrap(),
            fl.unwrap_or(2),
            sc.unwrap(),
        );
    }

    @noupgrade
    async constructor() {}
}

// --- Test --- //

import all_types.aleo;

program test_all_types.aleo {

    @test
    transition optional_basic_primitives() {
        let result = all_types.aleo/optional_basic_primitives();
        assert_eq(result, 52u32);
    }

    @test
    transition array_of_optionals() {
        let result = all_types.aleo/array_of_optionals();
        assert_eq(result, 24u8);
    }

    @test
    transition optional_array() {
        let result = all_types.aleo/optional_array();
        assert_eq(result, 1095u128);
    }

    @test
    transition optional_struct_field() {
        let result = all_types.aleo/optional_struct_field();
        assert_eq(result, 123u16);
    }

    @test
    transition optional_struct() {
        let result = all_types.aleo/optional_struct();
        assert_eq(result, 5u8);
    }

    @test
    transition array_of_optional_structs() {
        let result = all_types.aleo/array_of_optional_structs();
        assert_eq(result, 10u8);
    }

    @test
    transition nested_optional_structs() {
        let result = all_types.aleo/nested_optional_structs();
        assert_eq(result, 10u8);
    }

    @test
    transition tuples_with_optional_elements() {
        let result = all_types.aleo/tuples_with_optional_elements();
        assert_eq(result, 650u32);
    }

    @test
    transition optional_misc_primitive_types() {
        let (gr, fl, sc) = all_types.aleo/optional_misc_primitive_types();
        assert_eq(gr, 0group);
        assert_eq(fl, 1field);
        assert_eq(sc, 2scalar);
    }

    @noupgrade
    async constructor() {}
}
