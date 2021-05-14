/*
namespace: Compile
expectation: Fail
*/

circuit Foo {
    f: u32;

    function bar() -> u32 {
        return f;
    }
}

function main() -> u32 {
    const foo = Foo { f: 1u32 };
    const err = foo.bar();
}