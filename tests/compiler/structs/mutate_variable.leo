/*
namespace: Compile
expectation: Pass
input_file: input/dummy.in
*/

struct Foo {
    a: u8;
}

function main(y: bool) -> bool {
    let f = Foo { a: 0u8 };

    f.a = 1u8;
    f.a = 2u8;

    return (f.a == 2u8) == true;
}
