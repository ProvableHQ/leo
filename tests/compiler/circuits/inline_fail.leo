/*
namespace: Compile
expectation: Fail
*/

circuit Foo {
    x: u32
}

function main() {
    // no member y in Foo
    const a = Foo { y: 0u32 };
}
