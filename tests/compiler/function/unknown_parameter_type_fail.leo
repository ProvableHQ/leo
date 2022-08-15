/*
namespace: Compile
expectation: Fail
*/

@program
function main(a: u8, foo: Foo) -> u8 {
    return a;
}

@program
function returns_foo(a: u8) -> Foo {
    return a;
}