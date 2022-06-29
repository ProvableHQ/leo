/*
namespace: Compile
expectation: Fail
input_file: inputs/inline.in
*/

circuit Foo {
    x: u8,
	y: u8,
}

function main(x: u8) -> bool {
    const a: Foo = Foo { x, y: 0u8 };

    return true;
}