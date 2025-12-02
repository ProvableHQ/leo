program opt_bug.aleo {

    // --- Base Structs ---
    struct S { x: u32 }
    struct OS { inner: S? }
    struct Wrap { arr: [u32?; 2]?, val: u32? }

    // --- 1. Basic unwrap assigns implicitly to optional targets ---
    transition assign_optional_from_unwrap() -> u32 {
        let a: u32? = 5u32;

        // Case 1: T? = unwrap(T?)
        let b: u32? = a.unwrap();   // should implicitly wrap to u32?

        // Case 2: T? = unwrap(T?) + T
        let c: u32? = a.unwrap() + 7u32;

        // Case 3: T? = unwrap_or(T?, <unsuffixed>) + T
        let d: u32? = a.unwrap_or(0);

        return b.unwrap() + c.unwrap() + d.unwrap(); // 5 + 12 = 17
    }

    // --- 2. Unwrap inside arithmetic expressions (core regression) ---
    transition unwrap_inside_expression() -> u32 {
        let a: u32? = 9u32;
        let b = a.unwrap() * 2u32 + 3u32;
        return b; // 9*2 + 3 = 21
    }

    // --- 3. Deep nested expressions with multiple unwraps ---
    transition nested_expression_unwraps() -> u32 {
        let x: u32? = 4u32;
        let y: u32? = 6u32;
        let sum = x.unwrap() + (y.unwrap() * x.unwrap());
        return sum; // 4 + (6*4) = 28
    }

    // --- 4. Unwrap in arrays ---
    transition unwrap_in_arrays() -> u32 {
        let arr: [u32?; 3] = [1u32, none, 3u32];

        // unwrap_or in a composite expr
        let r = arr[0].unwrap()
              + arr[1].unwrap_or(10)
              + arr[2].unwrap();

        return r; // 1 + 10 + 3 = 14
    }

    // --- 5. Optional arrays themselves ---
    transition optional_array_handling() -> u32 {
        let arr: [u32?; 2]? = [7u32, none];
        let first = arr.unwrap()[0].unwrap();
        return first; // 7
    }

    // --- 6. Optional structs and fields inside expressions ---
    transition optional_struct_field_expr() -> u32 {
        let os: OS = OS { inner: S { x: 11u32 } };
        let result = os.inner.unwrap().x + 5u32;
        return result; // 16
    }

    // --- 7. Optional of a struct in arithmetic expression ---
    transition optional_struct_unwrap_assign() -> u32 {
        let s: S? = S { x: 30u32 };

        // Implicit wrapping back to optional
        let v: u32? = s.unwrap().x + 12u32;
        return v.unwrap(); // 42
    }

    // --- 8. Nested optional structs and arrays together ---
    transition nested_struct_array_mix() -> u32 {
        let w = Wrap { arr: [1u32, none], val: none };
        let arr_unwrapped = w.arr.unwrap();
        let r = arr_unwrapped[0].unwrap_or(0)
              + arr_unwrapped[1].unwrap_or(20u32);
        return r; // 1 + 20 = 21
    }

    // --- 9. Optional tuple fields ---
    transition tuple_optional_fields() -> u32 {
        let t: (u32?, u32) = (4u32, 9u32);
        let r = t.0.unwrap() + t.1;
        return r; // 4 + 9 = 13
    }

    // --- 10. Optional used inside function-like calls ---
    transition optionals_in_function_context() -> u32 {
        let a: u32? = 6u32;
        let b: u32? = none;

        // call-like layered expression
        let r = (a.unwrap() + 2u32) * (b.unwrap_or(4));
        return r; // (6+2) * 4 = 32
    }

    // --- 11. Stress: compound expression with 5+ unwraps ---
    transition stress_compound_unwrap() -> u32 {
        let a: u32? = 2u32;
        let b: u32? = 3u32;
        let c: u32? = none;

        let r = a.unwrap()
              + b.unwrap()
              + (a.unwrap() * b.unwrap())
              + c.unwrap_or(7u32)
              + (a.unwrap() + b.unwrap());

        // 2 + 3 + (2*3) + 7 + (2 + 3) = 2+3+6+7+5 = 23
        return r;
    }

    // --- 12. Ternary + Optionals ---
    transition ternary_optional_cases() -> u32 {
        let a: u32? = 10u32;
        let b: u32? = none;
        let cond = 1u32 < 5u32;   // true

        // Case 1: unwrap inside ternary branches
        let r1 = cond ? a.unwrap() : b.unwrap_or(99);
        // r1 = 10

        // Case 2: ternary returning an optional
        let r2: u32? = cond ? a.unwrap() + 5u32 : b.unwrap_or(7u32);
        // r2 = 15

        // Case 3: ternary consuming optional on one side only
        let r3 = cond ? r2.unwrap() * 2u32 : 3u32;
        // r3 = 30

        // Case 4: nested ternaries mixing optional/non-optional
        let r4 = cond
            ? (b != none ? b.unwrap() : a.unwrap() + 1u32)
            : 0u32;
        // b is none => r4 = 11

        // Case 5: ternary producing an optional
        let r5: u32? = cond ? r4 : 123;
        // r5 = 11

        return r1 + r2.unwrap() + r3 + r4 + r5.unwrap();
        // 10 + 15 + 30 + 11 + 11 = 77
    }

}
