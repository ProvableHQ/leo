circuit Foo {
    function echo(x: u32) -> u32 {
        return x
    }
}

function main() -> u32 {
    let c = Foo { };
    return c.echoed(1u32)
}