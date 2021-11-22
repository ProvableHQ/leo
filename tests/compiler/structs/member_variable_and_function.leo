/*
namespace: Compile
expectation: Pass
input_file: input/dummy.in
*/

struct Foo {
    foo: u32;

    function bar() -> u32 {
        return 1u32;
    }
}

function main(y: bool) -> bool {
    const a = Foo { foo: 1 };
    let b = a.foo + Foo::bar();

    return (b == 2u32) == y;
}
