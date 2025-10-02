program bad_types.aleo {
    struct Foo {
        x: u8,
    }

    struct Point {
        x: u8,
        y: u8,
    }

    struct MyStruct {
        val: u8,
    }

    // Optional array with mixed element types
    const BAD_CONST_ARRAY: [u8?; 2] = [1u8, 2i8]; // ERROR

    // === Basic assignment & comparison errors ===

    // ❌ Optional assigned to non-optional
    transition assign_optional_to_non_optional() {
        let a: u8? = 5u8;
        let b: u8 = a; // ERROR
    }

    // ❌ none assigned to non-optional
    transition assign_none_to_non_optional() {
        let a: u8 = none; // ERROR
    }

    // ❌ Optional<u8> compared to Optional<i8>
    transition compare_mismatched_types() {
        let a: u8? = 10u8;
        let b: i8? = 10i8;
        let result = a == b; // ERROR
    }

    // ❌ Optional compared to non-optional
    transition compare_opt_non_opt() {
        let a: u8? = 42u8;
        let b: u8 = 42u8;
        let result = a == b; // ERROR
    }

    struct Bar {
        val: u8?,
    }

    // ❌ Mixed types in optional array
    transition array_of_optionals_mixed_types() {
        let arr: [u8?; 3] = [1u8, none, 3i8]; // ERROR
    }

    // ❌ Optional struct compared to non-optional struct
    transition compare_struct_opt_to_non_opt() {
        let a: Foo = Foo { x: 1u8 };
        let b: Foo? = Foo { x: 1u8 };
        let result = a == b; // ERROR
    }

    // ❌ Assign optional array to non-optional array
    transition assign_opt_array_to_non_opt() {
        let a: [u8?; 2] = [1u8, 2u8];
        let b: [u8; 2] = a; // ERROR
    }

    // === Ternary errors ===

    // ❌ Mismatched branches in ternary
    transition ternary_mismatched_branches() {
        let flag: bool = true;
        let result = flag ? 10u8 : none; // ERROR
    }

    // ❌ Optional used as ternary condition
    transition ternary_optional_condition() {
        let condition: bool? = true;
        let x = condition ? 1u8 : 2u8; // ERROR
    }

    // === Inline function call errors ===

    // ❌ Inline fn expecting u8, given u8?
    inline takes_u8(x: u8) -> u8 {
        return x + 1u8;
    }

    transition call_fn_with_optional_arg() {
        let val: u8? = 5u8;
        let result = takes_u8(val); // ERROR
    }

    // ❌ Inline fn returning u8?, assigned to u8
    inline returns_optional() -> u8? {
        return 42u8;
    }

    transition assign_opt_return_to_non_opt() {
        let result: u8 = returns_optional(); // ERROR
    }

    // === Const-related errors ===

    @noupgrade
    async constructor() {}
}
