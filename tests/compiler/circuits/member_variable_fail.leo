/*
namespace: Compile
expectation: Fail
*/

circuit Foo {
    x: u32
}

function main() {
    const a = Foo { x: 1u32 };
    const err = a.y;
}
