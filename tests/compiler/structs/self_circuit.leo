/*
namespace: Compile
expectation: Fail
*/

struct Foo {
    // static is deprecated
    static function new() -> Self {
        return Self { };
    }
}

function main() {
    const a = Foo::new();
}
