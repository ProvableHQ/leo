program implicit_wrapping.aleo {
    // === Shared Structs ===

    struct Foo {
        x: u8?, // optional field
    }

    struct Wrapper {
        arr: [Foo?; 2], // array of optional structs
    }

    // === BASIC TYPE TESTS ===

    transition basic_implicit_return() -> u8? {
        return 5u8; // implicitly wrapped
    }

    inline takes_optional_u8(x: u8?) -> u8 {
        return x.unwrap_or(100u8);
    }

    transition basic_implicit_argument() -> u8 {
        return takes_optional_u8(42u8); // implicitly wrapped
    }

    transition basic_implicit_ternary(cond: bool) -> u8? {
        let result: u8? = cond ? 10u8 : None;
        return result; // inferred as u8?
    }

    transition basic_implicit_reassignment() -> u8 {
        let x: u8? = None;
        x = 99u8; // implicitly wrapped
        return x.unwrap();
    }

    // === COMPLEX TYPE TESTS ===

    // 1. Implicit return: Wrapper → Wrapper?
    transition complex_implicit_return() -> Wrapper? {
        return Wrapper {
            arr: [Foo { x: 8u8 }, None], // 8u8 implicitly wrapped into u8?
        };
    }

    // 2. Implicit argument: Wrapper → Wrapper?
    inline takes_optional_wrapper(w: Wrapper?) -> u8 {
        return w.unwrap().arr[0].unwrap().x.unwrap_or(0u8);
    }

    transition complex_implicit_argument() -> u8 {
        return takes_optional_wrapper(Wrapper {
            arr: [Foo { x: None }, None],
        });
    }

    // 3. Implicit ternary with Wrapper
    transition complex_implicit_ternary(cond: bool) -> Wrapper? {
        let result: Wrapper? = cond
            ? Wrapper { arr: [Foo { x: 99u8 }, None] } // implicit wrap
            : None;
        return result;
    }

    // 4. Implicit reassignment with Wrapper
    transition complex_implicit_reassignment() -> u8 {
        let w: Wrapper? = None;
        w = Wrapper { arr: [Foo { x: 7u8 }, None] }; // implicit wrap
        return w.unwrap().arr[0].unwrap().x.unwrap();
    }

    // 5. Implicit reassignment of [Wrapper; 3]? = [...]
    transition complex_array_wrapping() -> u8 {
        let arr_opt: [Wrapper; 3]? = None;
        arr_opt = [
            Wrapper { arr: [Foo { x: 1u8 }, None] },
            Wrapper { arr: [None, None] },
            Wrapper { arr: [Foo { x: 2u8 }, None] },
        ];
        return arr_opt.unwrap()[0].arr[0].unwrap().x.unwrap();
    }

    // 6. Deeper access into wrapped Wrapper array
    transition complex_array_access() -> u8 {
        let arr_opt: [Wrapper; 3]? = [
            Wrapper { arr: [Foo { x: 10u8 }, None] },
            Wrapper { arr: [Foo { x: 20u8 }, None] },
            Wrapper { arr: [None, None] },
        ];
        let w = arr_opt.unwrap()[1];
        let f = w.arr[0].unwrap();
        return f.x.unwrap(); // should be 20
    }
}
