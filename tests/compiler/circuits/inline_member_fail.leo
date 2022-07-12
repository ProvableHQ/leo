/*
namespace: Compile
expectation: Fail
*/

circuit Foo {
    x: u8;
}

function main() -> bool {
    const y: u8 = 1;
    const a: Foo = Foo { y };
    return true;
}
