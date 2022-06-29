/*
namespace: Compile
expectation: Fail
*/

circuit Foo {
    x: u32,
    y: u32,
}

function main() -> bool {
    const a: Foo = Foo { y: 0u32 };
    return true;
}
