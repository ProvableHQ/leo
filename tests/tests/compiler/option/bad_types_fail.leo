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

struct Bar {
    val: u8?,
}

// Optional array with mixed element types
const BAD_CONST_ARRAY: [u8?; 2] = [1u8, 2i8]; // ERROR

// ❌ Inline fn expecting u8, given u8?
fn takes_u8(x: u8) -> u8 {
    return x + 1u8;
}

// ❌ Inline fn returning u8?, assigned to u8
fn returns_optional() -> u8? {
    return 42u8;
}

program bad_types.aleo {
    // === Basic assignment & comparison errors ===

    // ❌ Optional assigned to non-optional
    fn assign_optional_to_non_optional() {
        let a: u8? = 5u8;
        let b: u8 = a; // ERROR
    }

    // ❌ none assigned to non-optional
    fn assign_none_to_non_optional() {
        let a: u8 = none; // ERROR
    }

    // ❌ Optional<u8> compared to Optional<i8>
    fn compare_mismatched_types() {
        let a: u8? = 10u8;
        let b: i8? = 10i8;
        let result = a == b; // ERROR
    }

    // ❌ Optional compared to non-optional
    fn compare_opt_non_opt() {
        let a: u8? = 42u8;
        let b: u8 = 42u8;
        let result = a == b; // ERROR
    }
    // ❌ Mixed types in optional array
    fn array_of_optionals_mixed_types() {
        let arr: [u8?; 3] = [1u8, none, 3i8]; // ERROR
    }

    // ❌ Optional struct compared to non-optional struct
    fn compare_struct_opt_to_non_opt() {
        let a: Foo = Foo { x: 1u8 };
        let b: Foo? = Foo { x: 1u8 };
        let result = a == b; // ERROR
    }

    // ❌ Assign optional array to non-optional array
    fn assign_opt_array_to_non_opt() {
        let a: [u8?; 2] = [1u8, 2u8];
        let b: [u8; 2] = a; // ERROR
    }

    // === Ternary errors ===

    // ❌ Mismatched branches in ternary
    fn ternary_mismatched_branches() {
        let flag: bool = true;
        let result = flag ? 10u8 : none; // ERROR
    }

    // ❌ Optional used as ternary condition
    fn ternary_optional_condition() {
        let condition: bool? = true;
        let x = condition ? 1u8 : 2u8; // ERROR
    }

    // === Inline function call errors ===

    fn call_fn_with_optional_arg() {
        let val: u8? = 5u8;
        let result = takes_u8(val); // ERROR
    }

    fn assign_opt_return_to_non_opt() {
        let result: u8 = returns_optional(); // ERROR
    }

    // === Const-related errors ===

    @noupgrade
    constructor() {}
}
