// Verifies that structs referenced only through type wrappers
// (Array, Optional, Mapping) are kept live by the unused-items pass.
// None of the structs below should emit a `struct is never constructed`
// warning, even though none of them is ever constructed in user code.

// Used as the element type of an `Array` in the entry-point signature.
struct InArr {
    x: u32,
}

// Used as the inner of an `Optional` input on a free fn called from main.
struct InOpt {
    x: u32,
}

// Used as the inner of an `Optional` return type on a free fn called from main.
struct InRet {
    x: u32,
}

// Used as the value type of a mapping.
struct InMap {
    x: u32,
}

// Free fn taking an `Optional<InOpt>`. Called from main, so the function is
// live and its signature is walked. `_opt` is silenced by the leading `_`.
fn helper_opt(_opt: InOpt?) -> u32 {
    return 0u32;
}

// Free fn returning `Optional<InRet>`. Called from main, so the function is
// live and its signature is walked.
fn helper_ret() -> InRet? {
    return none;
}

program test.aleo {
    mapping balances: address => InMap;

    fn main(arr: [InArr; 1]) -> u32 {
        let _r = helper_ret();
        return arr[0u32].x + helper_opt(none);
    }

    @noupgrade
    constructor() {}
}
