/*
namespace: Compile
expectation: Fail
*/

circuit Foo {
    // static is deprecated
    static function new() -> Self {
        return Self { };
    }
}

function main() {
    const a = Foo::new();
}
