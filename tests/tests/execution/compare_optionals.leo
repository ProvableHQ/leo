/*
seed = 2468101214

[case]
program = "compare_optionals.aleo"
function = "u8_some_eq_none"
input = []

[case]
program = "compare_optionals.aleo"
function = "i128_none_eq_none"
input = []

[case]
program = "compare_optionals.aleo"
function = "u8_some_eq_some"
input = []

[case]
program = "compare_optionals.aleo"
function = "i128_some_neq_some"
input = []

[case]
program = "compare_optionals.aleo"
function = "struct_some_eq_some"
input = []

[case]
program = "compare_optionals.aleo"
function = "struct_some_neq_none"
input = []

[case]
program = "compare_optionals.aleo"
function = "array_some_eq_some"
input = []

[case]
program = "compare_optionals.aleo"
function = "array_some_neq_some"
input = []
*/

struct Point {
    x: u8,
    y: u8,
}

program compare_optionals.aleo {
    // u8? == none → false
    fn u8_some_eq_none() -> bool {
        let val: u8? = 5u8;
        return val == none;
    }

    // i128? == none → true (comparing two none values)
    fn i128_none_eq_none() -> bool {
        let a: i128? = none;
        let b: i128? = none;
        return a == b;
    }

    // u8? == u8? (same value) → true
    fn u8_some_eq_some() -> bool {
        let a: u8? = 42u8;
        let b: u8? = 42u8;
        return a == b;
    }

    // i128? != i128? (different values) → true
    fn i128_some_neq_some() -> bool {
        let a: i128? = 100i128;
        let b: i128? = -50i128;
        return a != b;
    }

    // struct? == struct? (same fields) → true
    fn struct_some_eq_some() -> bool {
        let a: Point? = Point { x: 1u8, y: 2u8 };
        let b: Point? = Point { x: 1u8, y: 2u8 };
        return a == b;
    }

    // struct? != none → true
    fn struct_some_neq_none() -> bool {
        let a: Point? = Point { x: 5u8, y: 9u8 };
        return a != none;
    }

    // [u8; 2]? == [u8; 2]? → true
    fn array_some_eq_some() -> bool {
        let a: [u8; 2]? = [7u8, 8u8];
        let b: [u8; 2]? = [7u8, 8u8];
        return a == b;
    }

    // [u8; 2]? != [u8; 2]? → true
    fn array_some_neq_some() -> bool {
        let a: [u8; 2]? = [1u8, 2u8];
        let b: [u8; 2]? = [3u8, 4u8];
        return a != b;
    }

    @noupgrade
    constructor() {}
}

