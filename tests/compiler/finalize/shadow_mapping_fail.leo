/*
namespace: Compile
expectation: Fail
*/


mapping bar: u8 => u8;

@program
function bar(a: u8) -> u8 {
    return a + 1u8;
}

@program
function foo(bar: u8) -> u8 {
    return bar + 1u8;
}

@program
function baz(a: u8) -> u8 {
    let bar: u8 = a + 1u8;
    return bar;
}

circuit bar {
    a: u64;
}
