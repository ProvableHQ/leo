/*
namespace: Compile
expectation: Pass
input_file: input/dummy.in
*/

circuit Foo {
    x: u32,
}

function main(y: bool) -> bool {
    const a = Foo { x: 1u32 };

    return a.x == 1u32 == y;
}
