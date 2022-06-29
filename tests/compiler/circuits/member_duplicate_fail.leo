/*
namespace: Compile
expectation: Fail
input_file: inputs/const_inline.in
*/

circuit Foo {
    x: u8
}

function main(const x: u8, y: bool) -> bool {
    const a: Foo = Foo { x: 0u8, x: 1u8 };

    return (a.x == a.x) == y;
}

