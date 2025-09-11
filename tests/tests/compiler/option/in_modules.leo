program in_modules.aleo {
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

    // 1. Implicit return: dep::Wrapper → dep::Wrapper?
    transition complex_implicit_return() -> dep::Wrapper? {
        return dep::Wrapper {
            arr: [dep::inner::Foo { x: 8u8 }, None], // 8u8 implicitly wrapped into u8?
        };
    }

    // 2. Implicit argument: dep::Wrapper → dep::Wrapper?
    inline takes_optional_wrapper(w: dep::Wrapper?) -> u8 {
        return w.unwrap().arr[0].unwrap().x.unwrap_or(0u8);
    }

    transition complex_implicit_argument() -> u8 {
        return takes_optional_wrapper(dep::Wrapper {
            arr: [dep::inner::Foo { x: None }, None],
        });
    }

    // 3. Implicit ternary with dep::Wrapper
    transition complex_implicit_ternary(cond: bool) -> dep::Wrapper? {
        let result: dep::Wrapper? = cond
            ? dep::Wrapper { arr: [dep::inner::Foo { x: 99u8 }, None] } // implicit wrap
            : None;
        return result;
    }

    // 4. Implicit reassignment with dep::Wrapper
    transition complex_implicit_reassignment() -> u8 {
        let w: dep::Wrapper? = None;
        w = dep::Wrapper { arr: [dep::inner::Foo { x: 7u8 }, None] }; // implicit wrap
        return w.unwrap().arr[0].unwrap().x.unwrap();
    }

    // 5. Implicit reassignment of [dep::Wrapper; 3]? = [...]
    transition complex_array_wrapping() -> u8 {
        let arr_opt: [dep::Wrapper; 3]? = None;
        arr_opt = [
            dep::Wrapper { arr: [dep::inner::Foo { x: 1u8 }, None] },
            dep::Wrapper { arr: [None, None] },
            dep::Wrapper { arr: [dep::inner::Foo { x: 2u8 }, None] },
        ];
        return arr_opt.unwrap()[0].arr[0].unwrap().x.unwrap();
    }

    // 6. Deeper access into wrapped dep::Wrapper array
    transition complex_array_access() -> u8 {
        let arr_opt: [dep::Wrapper; 3]? = [
            dep::Wrapper { arr: [dep::inner::Foo { x: 10u8 }, None] },
            dep::Wrapper { arr: [dep::inner::Foo { x: 20u8 }, None] },
            dep::Wrapper { arr: [None, None] },
        ];
        let w = arr_opt.unwrap()[1];
        let f = w.arr[0].unwrap();
        return f.x.unwrap(); // should be 20
    }
}

// --- Next Module: dep.leo --- //

struct Wrapper {
    arr: [inner::Foo?; 2], // array of optional structs
}

// --- Next Module: dep/inner.leo --- //

struct Foo {
    x: u8?, // optional field
}
