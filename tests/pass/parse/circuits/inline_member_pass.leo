circuit Foo {
    x: u8

    function new(x: u8) -> Self {
        return Self { x }
    }
}

function main() {
    let x: u8 = 1;
    let a = Foo { x };
    let b = Foo::new(x);
}