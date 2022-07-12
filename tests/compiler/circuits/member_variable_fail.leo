/*
namespace: Compile
expectation: Fail
*/

circuit Foo {
    x: u32;
}

function main() -> bool {
    const a: foo = Foo { x: 1u32 };
    const err = a.y;
    return true;
}
