
struct Foo {
    x: u8
}

program test.aleo {
    function main(let x: u8, y: bool) -> bool {
        let a: Foo = Foo { x };

        return (b.x == a.x) == y;
    }
}
