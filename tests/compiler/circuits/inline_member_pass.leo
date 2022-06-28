/*
namespace: Compile
expectation: Pass
input_file: inputs/const_inline.in
*/

circuit Foo {
    x: u8
}

function main(const x: u8, y: bool) -> bool {
    const a: Foo = Foo { x };

    return (a.x == a.x) == y;
}
