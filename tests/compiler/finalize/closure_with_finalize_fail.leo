/*
namespace: Compile
expectation: Fail
*/

function foo(a: u8, b: u8) -> u8 {
    finalize(a, b);
    return a + b;
}

function bar(a: u8, b: u8) -> u8 {
    return a + b;
}

finalize bar(a: u8, b: u8) -> u8 {
    return a + b;
}


function mint_public(receiver: address, amount: u64) {
    finalize(receiver, amount);
}

finalize mint_public(receiver: address, amount: u64) {
    increment(account, receiver, amount);
}

