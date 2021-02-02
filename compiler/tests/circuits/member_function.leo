circuit Foo {
    x: u32,

    function echo(self) -> u32 {
        return self.x
    }
}

function main() {
    let a = Foo { x: 1u32 };

    console.assert(a.echo() == 1u32);
}