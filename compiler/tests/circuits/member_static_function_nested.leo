circuit Foo {
    static function qux() {}

    static function bar() {
        Self::qux();
    }

    static function baz() {
        Self::bar();
    }
}

function main() {
    Foo::baz();
}