/*
namespace: Compile
expectation: Pass
input_file: input/dummy.in
*/

circuit Foo {
    x: u32
}

// Using let makes a circuit variable mutable.
function main(y: bool) -> bool{
    let a = Foo { x: 1 };
    a.x = 0;

    return (a.x == 0u32) == y;
}
