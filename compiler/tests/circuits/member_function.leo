circuit Foo {
    function echo(x: u32) -> u32 {
        return x
    }
}

function main() {
    let a = Foo { };

    assert_eq!(a.echo(1u32), 1u32);
}