/*
namespace: Compile
expectation: Fail
*/

circuit Foo {
    function bar() -> u32 {
        return self.f;
    }
}

function main() {
    const foo = Foo { };
    const err = foo.bar();
}