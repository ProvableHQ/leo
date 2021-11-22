/*
namespace: Compile
expectation: Pass
input_file: input/dummy.in
*/

struct Foo {
    function echo(x: u32) -> u32 {
        return x;
    }
}

function main(y: bool) -> bool {
    let a = Foo::echo(1u32);

    return (a == 1u32) == y;
}
