/*
namespace: Compile
expectation: Pass
input_file: inputs/inline.in
*/

circuit Foo {
    x: u32
}

function main(x: u32) -> u32 {
    let a: Foo = Foo { x };
    return a.x;
}
