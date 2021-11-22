/*
namespace: Compile
expectation: Fail
input_file: input/dummy.in
*/

struct Foo {
    static const x: u32 = 10;
}

function main(y: bool) -> bool {
    let f = Foo {};
    f.x = 0;
    return y;
}
