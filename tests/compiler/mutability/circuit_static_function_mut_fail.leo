/*
namespace: Compile
expectation: Fail
*/

circuit Foo {
    function bar() {}
}

function main() {
    let a = Foo { x: 1 };
    a.bar = 0;
}
