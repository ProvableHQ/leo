circuit Foo {
    static function echo(x: u32) -> u32 {
        return x
    }
}

function main() -> u32 {
    let c = Foo { };
    return c.echo(1u32) // echo is a static function and must be accessed using `::`
}