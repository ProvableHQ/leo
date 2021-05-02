circuit Foo {
    function echo(self, x: u32) -> u32 {
        return x;
    }
}

function main() {
    const a = Foo { };

    console.assert(a.echo(1u32) == 1u32);
}