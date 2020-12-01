circuit Foo {
    function echo(x: u32) -> u32 {
        return x
    }
}

function main() {
    let err = Foo::echo(1u32); // Correct, echo is a static function and must be accessed using `::`
}