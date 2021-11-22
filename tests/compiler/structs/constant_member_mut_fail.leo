/*
namespace: Compile
expectation: Fail
input_file: input/dummy.in
*/

struct Foo {
    static const y: u32 = 23;
}

function main(y: bool) -> bool {
    Foo::y = 0;
    return y;
}
