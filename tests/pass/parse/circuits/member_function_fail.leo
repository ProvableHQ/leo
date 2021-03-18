circuit Foo {
    function echo(x: u32) -> u32 {
        return x
    }
}

function main() {
    const a = Foo { };
    const err = a.echoed(1u32);
}