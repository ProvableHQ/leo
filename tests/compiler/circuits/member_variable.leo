/*
namespace: Compile
expectation: Pass
input_file: inputs/dummy.in
*/

circuit Foo {
    x: u32,
    y: u8,
}

function main(y: bool) -> bool {
    const a: Foo = Foo { x: 1u32, y: 0u8 };

    return (a.x == 1u32) && (a.y == 0u8) == y;
}
