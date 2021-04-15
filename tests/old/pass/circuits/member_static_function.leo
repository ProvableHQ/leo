circuit Foo {
    function echo(x: u32) -> u32 {
        return x;
    }
}

function main() {
    const a = Foo::echo(1u32);

    console.assert(a == 1u32);
}