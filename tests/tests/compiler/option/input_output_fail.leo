struct Bar { x: u8? }
struct Foo { b: Bar }

program test.aleo {
    fn foo(x: u8?, y: [u8?; 3], f: Foo) -> (u8?, u8?, Foo) {
        return (x, x, f);
    }

    fn bar(x: u8?, f: Foo) {
        return final {};
    }
}

@no_inline
fn baz(x: u8?) -> u8? {
    return x;
}
