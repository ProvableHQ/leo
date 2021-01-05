circuit Foo {
    function qux() {}

    function bar() {
        Self::qux();
    }

    function baz() {
        Self::bar();
    }
}

function main() {
    Foo::baz();
}