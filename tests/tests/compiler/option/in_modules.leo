program in_modules.aleo {
    // === BASIC TYPE TESTS ===

    // 1. Return u8? → move to inline, unwrap in transition
    inline basic_implicit_return_() -> u8? {
        return 5u8; // implicitly wrapped
    }

    transition basic_implicit_return() -> u8 {
        return basic_implicit_return_().unwrap();
    }

    // 2. Optional input (inline is OK)
    inline takes_optional_u8(x: u8?) -> u8 {
        return x.unwrap_or(100u8);
    }

    // 3. Implicit optional argument → use inline
    inline basic_implicit_argument_() -> u8 {
        return takes_optional_u8(42u8); // implicitly wrapped
    }

    transition basic_implicit_argument() -> u8 {
        return basic_implicit_argument_();
    }

    // 4. Return u8? → unwrap
    inline basic_implicit_ternary_(cond: bool) -> u8? {
        let result: u8? = cond ? 10u8 : none;
        return result;
    }

    transition basic_implicit_ternary(cond: bool) -> u8 {
        return basic_implicit_ternary_(cond).unwrap();
    }

    // 5. Only optional reassignment — OK
    transition basic_implicit_reassignment() -> u8 {
        let x: u8? = none;
        x = 99u8;
        return x.unwrap();
    }

    // === COMPLEX TYPE TESTS ===

    // 1. Return dep::Wrapper? → unwrap fully to [u8; 2]
    inline complex_implicit_return_() -> [u8?; 2] {
        let wrapper_opt: dep::Wrapper? = dep::Wrapper {
            arr: [dep::inner::Foo { x: 8u8 }, none],
        };
        let wrapper = wrapper_opt.unwrap();
        return [
            wrapper.arr[0].unwrap().x,
            wrapper.arr[1].unwrap_or(dep::inner::Foo { x: none }).x,
        ];
    }

    transition complex_implicit_return() -> [u8; 2] {
        let xs = complex_implicit_return_();
        return [xs[0].unwrap(), xs[1].unwrap_or(0u8)];
    }

    // 2. Optional arg → already inline
    inline takes_optional_wrapper(w: dep::Wrapper?) -> u8 {
        return w.unwrap().arr[0].unwrap().x.unwrap_or(0u8);
    }

    inline complex_implicit_argument_() -> u8 {
        return takes_optional_wrapper(dep::Wrapper {
            arr: [dep::inner::Foo { x: none }, none],
        });
    }

    transition complex_implicit_argument() -> u8 {
        return complex_implicit_argument_();
    }

    // 3. Return dep::Wrapper? → unwrap fully to [u8; 2]
    inline complex_implicit_ternary_(cond: bool) -> [u8?; 2] {
        let wrapper_opt: dep::Wrapper? = cond
            ? dep::Wrapper { arr: [dep::inner::Foo { x: 99u8 }, none] }
            : none;
        let wrapper = wrapper_opt.unwrap();
        return [
            wrapper.arr[0].unwrap().x,
            wrapper.arr[1].unwrap_or(dep::inner::Foo { x: none }).x,
        ];
    }

    transition complex_implicit_ternary(cond: bool) -> [u8; 2] {
        let xs = complex_implicit_ternary_(cond);
        return [xs[0].unwrap_or(0u8), xs[1].unwrap_or(0u8)];
    }

    // 4. Optional reassignment only — allowed
    transition complex_implicit_reassignment() -> u8 {
        let w: dep::Wrapper? = none;
        w = dep::Wrapper { arr: [dep::inner::Foo { x: 7u8 }, none] };
        return w.unwrap().arr[0].unwrap().x.unwrap();
    }

    // 5. [dep::Wrapper; 3]? → unwrap all the way to [[u8; 2]; 3]
    inline complex_array_wrapping_() -> [[u8?; 2]; 3] {
        let arr_opt: [dep::Wrapper; 3]? = none;
        let arr = arr_opt.unwrap();

        return [
            [
                arr[0].arr[0].unwrap().x,
                arr[0].arr[1].unwrap_or(dep::inner::Foo { x: none }).x,
            ],
            [
                arr[1].arr[0].unwrap_or(dep::inner::Foo { x: none }).x,
                arr[1].arr[1].unwrap_or(dep::inner::Foo { x: none }).x,
            ],
            [
                arr[2].arr[0].unwrap().x,
                arr[2].arr[1].unwrap_or(dep::inner::Foo { x: none }).x,
            ],
        ];
    }

    transition complex_array_wrapping() -> u8 {
        let arr = complex_array_wrapping_();
        return arr[0][0].unwrap_or(0u8); // only return one value
    }

    // 6. Deeper access into optional array — unwrap fully
    inline complex_array_access_() -> [[u8?; 2]; 3] {
        let arr_opt: [dep::Wrapper; 3]? = [
            dep::Wrapper { arr: [dep::inner::Foo { x: 10u8 }, none] },
            dep::Wrapper { arr: [dep::inner::Foo { x: 20u8 }, none] },
            dep::Wrapper { arr: [none, none] },
        ];
        let arr = arr_opt.unwrap();
        return [
            [
                arr[0].arr[0].unwrap().x,
                arr[0].arr[1].unwrap_or(dep::inner::Foo { x: none }).x,
            ],
            [
                arr[1].arr[0].unwrap().x,
                arr[1].arr[1].unwrap_or(dep::inner::Foo { x: none }).x,
            ],
            [
                arr[2].arr[0].unwrap_or(dep::inner::Foo { x: none }).x,
                arr[2].arr[1].unwrap_or(dep::inner::Foo { x: none }).x,
            ],
        ];
    }

    transition complex_array_access() -> u8 {
        let arr = complex_array_access_();
        return arr[1][0].unwrap(); // should be 20
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
