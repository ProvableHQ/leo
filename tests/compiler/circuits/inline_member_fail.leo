/*
namespace: Compile
expectation: Fail
*/

circuit Foo {
    x: u8
}

function main() {
    const y: u8 = 1;
    const a = Foo { y };
}
