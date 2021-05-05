/*
namespace: Compile
expectation: Fail
*/

circuit Foo {
    function echo(x: u32) -> u32 {
        return x;
    }
}

function main() {
    const a = Foo { };
    const err = a.echo(1u32); // echo is a static function and must be accessed using `::`
}
