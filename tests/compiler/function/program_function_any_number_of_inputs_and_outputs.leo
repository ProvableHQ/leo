/*
namespace: Compile
expectation: Pass
*/

@program
function foo0_to_0() {}

@program
function foo0_to_1() -> u8 {
    return 1u8 + 1u8;
}

@program
function foo1_to_0(a: u8) {}

@program
function foo1_to_1(a: u8) -> u8 {
    return a * a;
}
