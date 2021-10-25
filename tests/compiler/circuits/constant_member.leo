/*
namespace: Compile
expectation: Pass
input_file: input/dummy.in
*/

circuit Foo {
    static const x: u32 = 4u32;
    static const y: u32 = 23;
}

function main(y: bool) -> bool {
    return (Foo::x == 4u32) && (Foo::y == 23u32) == y;
}
