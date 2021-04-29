circuit Foo {
    x: u8;

    function new(x: u8) -> Self {
        return Self { x };
    }
}

function main() {
    const x: u8 = 1;
    const a = Foo { x };
    const b = Foo::new(x);
}