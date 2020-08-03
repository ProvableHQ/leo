// Functions inputs in leo are pass-by-value.
//
// Program execution:
//   line 15: variable `a` is defined with value `1`.
//   line 16: value `1` is copied and passed into function `bad_mutate()`.
//   line 10: variable `x` is defined with value `1`.
//   line 11: variable `x` is set to value `0`.
//   line 18: program returns the value of `a`.

function bad_mutate(mut x: u32) {
    x = 0; // <- does not change `a`
}

function main() {
    let a = 1u32;
    bad_mutate(a);

    assert_eq!(a, 1u32); // <- value `a` is still `1u32`
}