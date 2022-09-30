/*
namespace: Compile
expectation: Fail
*/

struct Foo {
    x: u32;
}

function main() {
    const a: foo = Foo { x: 1u32 };
    const err = a.y;
}
