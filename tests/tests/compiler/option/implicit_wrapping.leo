program implicit_wrapping.aleo {
    // === Shared Structs ===

    struct Foo {
        x: u8?, // optional field
    }

    struct Wrapper {
        arr: [Foo?; 2], // array of optional structs
    }

    // === BASIC TYPE TESTS ===

    // 1. Return u8? → move logic to inline
    inline basic_implicit_return_() -> u8? {
        return 5u8; // implicitly wrapped
    }

    transition basic_implicit_return() -> u8 {
        return basic_implicit_return_().unwrap();
    }

    // 2. Optional input (inline is allowed)
    inline takes_optional_u8(x: u8?) -> u8 {
        return x.unwrap_or(100u8);
    }

    // 3. Optional argument passed → use inline
    inline basic_implicit_argument_() -> u8 {
        return takes_optional_u8(42u8); // implicitly wrapped
    }

    transition basic_implicit_argument() -> u8 {
        return basic_implicit_argument_();
    }

    // 4. Return u8? → move logic to inline
    inline basic_implicit_ternary_(cond: bool) -> u8? {
        let result: u8? = cond ? 10u8 : none;
        return result;
    }

    transition basic_implicit_ternary(cond: bool) -> u8 {
        return basic_implicit_ternary_(cond).unwrap();
    }

    // 5. Optional reassignment only (return is concrete) → allowed
    transition basic_implicit_reassignment() -> u8 {
        let x: u8? = none;
        x = 99u8; // implicitly wrapped
        return x.unwrap();
    }

    // === COMPLEX TYPE TESTS ===

    // 1. Return Wrapper? → move to inline
    inline complex_implicit_return_() -> Wrapper? {
        return Wrapper {
            arr: [Foo { x: 8u8 }, none],
        };
    }

    transition complex_implicit_return() -> [u8; 2] {
        let w = complex_implicit_return_().unwrap();
        return [w.arr[0].unwrap().x.unwrap(), w.arr[1].unwrap().x.unwrap()];
    }

    // 2. Optional argument to inline → wrap call
    inline takes_optional_wrapper(w: Wrapper?) -> u8 {
        return w.unwrap().arr[0].unwrap().x.unwrap_or(0u8);
    }

    inline complex_implicit_argument_() -> u8 {
        return takes_optional_wrapper(Wrapper {
            arr: [Foo { x: none }, none],
        });
    }

    transition complex_implicit_argument() -> u8 {
        return complex_implicit_argument_();
    }

    // 3. Return Wrapper? → move to inline
    inline complex_implicit_ternary_(cond: bool) -> Wrapper? {
        let result: Wrapper? = cond
            ? Wrapper { arr: [Foo { x: 99u8 }, none] }
            : none;
        return result;
    }

    transition complex_implicit_ternary(cond: bool) -> [u8; 2] {
        let w = complex_implicit_ternary_(cond).unwrap();
        return [w.arr[0].unwrap().x.unwrap(), w.arr[1].unwrap().x.unwrap()];
    }

    // 4. Optional reassignment only → allowed
    transition complex_implicit_reassignment() -> u8 {
        let w: Wrapper? = none;
        w = Wrapper { arr: [Foo { x: 7u8 }, none] };
        return w.unwrap().arr[0].unwrap().x.unwrap();
    }

    // 5. Return is concrete but input is optional → move to inline
    inline complex_array_wrapping_() -> u8 {
        let arr_opt: [Wrapper; 3]? = none;
        arr_opt = [
            Wrapper { arr: [Foo { x: 1u8 }, none] },
            Wrapper { arr: [none, none] },
            Wrapper { arr: [Foo { x: 2u8 }, none] },
        ];
        return arr_opt.unwrap()[0].arr[0].unwrap().x.unwrap();
    }

    transition complex_array_wrapping() -> u8 {
        return complex_array_wrapping_();
    }

    // 6. Same: optional input, concrete return → move to inline
    inline complex_array_access_() -> u8 {
        let arr_opt: [Wrapper; 3]? = [
            Wrapper { arr: [Foo { x: 10u8 }, none] },
            Wrapper { arr: [Foo { x: 20u8 }, none] },
            Wrapper { arr: [none, none] },
        ];
        let w = arr_opt.unwrap()[1];
        let f = w.arr[0].unwrap();
        return f.x.unwrap(); // should be 20
    }

    transition complex_array_access() -> u8 {
        return complex_array_access_();
    }
}
