/*
seed = 20250911

[case]
program = "implicit_wrapping.aleo"
function = "basic_implicit_return"
input = []

[case]
program = "implicit_wrapping.aleo"
function = "basic_implicit_argument"
input = []

[case]
program = "implicit_wrapping.aleo"
function = "basic_implicit_ternary"
input = ["true"]

[case]
program = "implicit_wrapping.aleo"
function = "basic_implicit_ternary"
input = ["false"]

[case]
program = "implicit_wrapping.aleo"
function = "basic_implicit_reassignment"
input = []

[case]
program = "implicit_wrapping.aleo"
function = "complex_implicit_return"
input = []

[case]
program = "implicit_wrapping.aleo"
function = "complex_implicit_argument"
input = []

[case]
program = "implicit_wrapping.aleo"
function = "complex_implicit_ternary"
input = ["true"]

[case]
program = "implicit_wrapping.aleo"
function = "complex_implicit_reassignment"
input = []

[case]
program = "implicit_wrapping.aleo"
function = "complex_array_wrapping"
input = []

[case]
program = "implicit_wrapping.aleo"
function = "complex_array_access"
input = []
*/

program implicit_wrapping.aleo {
    // === Shared Structs ===

    struct Foo {
        x: u8?, // optional field
    }

    struct Wrapper {
        arr: [Foo?; 2], // array of optional structs
    }

    // === BASIC TYPE TESTS ===

    transition basic_implicit_return() -> u8 {
        return foo().unwrap();
    }

    inline foo() -> u8? {
        return 5; // implicitly wrapped
    }

    inline takes_optional_u8(x: u8?) -> u8 {
        return x.unwrap_or(100);
    }

    transition basic_implicit_argument() -> u8 {
        return takes_optional_u8(42); // implicitly wrapped
    }

    transition basic_implicit_ternary(cond: bool) -> u8 {
        let result: u8? = cond ? 10 : none;
        return result.unwrap_or(42u8); // inferred as u8?
    }

    struct Bar {
        x: [u8?; 3]
    }

    transition basic_implicit_reassignment() -> u8 {
        let b: Bar = Bar { x: [none, none, none] };
        b.x[0] = 99; // implicitly wrapped
        return b.x[0].unwrap();
    }

    // === COMPLEX TYPE TESTS ===

    // 1. Implicit return: Wrapper → Wrapper?
    transition complex_implicit_return() -> u8 {
        return bar().unwrap().arr[0].unwrap().x.unwrap();
    }

    inline bar() -> Wrapper? {
        return Wrapper {
            arr: [Foo { x: 8 }, none], // 8u8 implicitly wrapped into u8?
        };
    }

    // 2. Implicit argument: Wrapper → Wrapper?
    inline takes_optional_wrapper(w: Wrapper?) -> u8 {
        return w.unwrap().arr[0].unwrap().x.unwrap_or(42);
    }

    transition complex_implicit_argument() -> u8 {
        return takes_optional_wrapper(Wrapper {
            arr: [Foo { x: none }, none],
        });
    }

    // 3. Implicit ternary with Wrapper
    transition complex_implicit_ternary(cond: bool) -> u8 {
        let result: Wrapper? = cond
            ? Wrapper { arr: [Foo { x: 99 }, none] } // implicit wrap
            : none;
        return result.unwrap().arr[0].unwrap().x.unwrap_or(69);
    }

    // 4. Implicit reassignment with Wrapper
    transition complex_implicit_reassignment() -> u8 {
        let w: Wrapper? = none;
        w = Wrapper { arr: [Foo { x: 7 }, none] }; // implicit wrap
        return w.unwrap().arr[0].unwrap().x.unwrap();
    }

    // 5. Implicit reassignment of [Wrapper; 3]? = [...]
    transition complex_array_wrapping() -> u8 {
        let arr_opt: [Wrapper; 3]? = none;
        arr_opt = [
            Wrapper { arr: [Foo { x: 1 }, none] },
            Wrapper { arr: [none, none] },
            Wrapper { arr: [Foo { x: 2 }, none] },
        ];
        return arr_opt.unwrap()[0].arr[0].unwrap().x.unwrap();
    }

    // 6. Deeper access into wrapped Wrapper array
    transition complex_array_access() -> u8 {
        let arr_opt: [Wrapper; 3]? = [
            Wrapper { arr: [Foo { x: 10 }, none] },
            Wrapper { arr: [Foo { x: 20 }, none] },
            Wrapper { arr: [none, none] },
        ];
        let w = arr_opt.unwrap()[1];
        let f = w.arr[0].unwrap();
        return f.x.unwrap(); // should be 20
    }

    @noupgrade
    async constructor() {}
}
