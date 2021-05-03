/*
namespace: Compile
expectation: Pass
*/

circuit Foo {
    x: u32
}

// Using let makes a circuit variable mutable.
function main() {
    let a = Foo { x: 1 };
    a.x = 0;

    console.assert(a.x == 0u32);
}