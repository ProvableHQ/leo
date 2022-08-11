/*
namespace: Compile
expectation: Fail
*/

circuit Foo {
    a: u8,
    bar: Bar,
}

@program
function main(a: u8) -> u8 {
    return a + 1u8;
}
