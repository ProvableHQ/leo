circuit Foo {
    function echo(x: u32) -> u32 {
        return x
    }
}

function main() -> u32 {
    return Foo::echo(1u32) // echo is a non-static function and must be accessed using `.`
}