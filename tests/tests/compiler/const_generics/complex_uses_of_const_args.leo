program complex.aleo {
    @noupgrade
    async constructor() {}

    // A generic struct with a const parameter
    struct Vector::[LEN: u32] {
        values: [u32; LEN],
    }

    // A generic function that returns a struct with const-evaluated param
    inline make_vector::[N: u32]() -> Vector::[N + 3] {
        return Vector::[N + 3] { values: [1u32; N + 3] };
    }

    // A more complex generic function
    inline make_offset_vector::[A: u32, B: u32]() -> Vector::[(A * 2 + B / 2) - 1] {
        return Vector::[(A * 2 + B / 2) - 1] {
            values: [9u32; (A * 2 + B / 2) - 1],
        };
    }

    // Uses const expression with nested arithmetic
    transition complex_call_pass() -> Vector::[8] {
        return make_offset_vector::[4, 2]();  // (4 * 2 + 2 / 2) - 1 = 8
    }

    // Combine literal and arithmetic expressions directly
    transition inline_nested_pass() -> Vector::[13] {
        return make_vector::[(2 + 8)]();  // 2 + 8 + 3 = 13
    }

    // Higher complexity with shared computed expression
    inline repeat_value::[COUNT: u32](val: u32) -> [u32; COUNT * 2] {
        return [val; COUNT * 2];
    }

    transition call_repeat() -> [u32; 6] {
        return repeat_value::[3](42);
    }
}
