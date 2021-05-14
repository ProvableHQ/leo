/*
namespace: Compile
expectation: Fail
*/

circuit Foo {
    x: u32;
}

// const variable is immutable
function main() {
    const a = Foo { x: 1 };
    a.x = 0;
}