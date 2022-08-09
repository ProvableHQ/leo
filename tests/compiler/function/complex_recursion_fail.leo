/*
namespace: Compile
expectation: Fail
*/

@inline
function one(n: u8) -> u8 {
    return two(n);
}

@inline
function two(n: u8) -> u8 {
    return three(n) + four(n);
}

@inline three(n: u8) -> u8 {
    return one(n);
}

@inline four(n: u8) -> u8 {
    return one(n);
}