/*
namespace: Compile
expectation: Pass
input_file: input/dummy.in
*/

// Functions input in leo are pass-by-value.
//
// Program execution:
//   line 15: variable `a` is defined with value `1`.
//   line 16: value `1` is copied and passed into function `bad_mutate()`.
//   line 10: variable `x` is defined with value `1`.
//   line 11: variable `x` is set to value `0`.
//   line 18: program returns the value of `a`.

function bad_mutate(x: u32) {
    x = 0; // <- does not change `a`
}

function main(y: bool) -> bool {
    const a = 1u32;
    bad_mutate(a);

    return a == 1u32 == y; // <- value `a` is still `1u32`
}
