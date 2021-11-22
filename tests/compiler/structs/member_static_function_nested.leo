/*
namespace: Compile
expectation: Pass
input_file: input/dummy.in
*/

struct Foo {
    function qux() {}

    function bar() {
        Self::qux();
    }

    function baz() {
        Self::bar();
    }
}

function main(y: bool) -> bool {
    Foo::baz();

    return y == true;
}
