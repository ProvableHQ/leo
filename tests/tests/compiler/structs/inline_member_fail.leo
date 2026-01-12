
struct Foo {
    x: u8,
}

program test.aleo {
    function main() {
        let y: u8 = 1;
        let a: Foo = Foo { y };
    }
}
