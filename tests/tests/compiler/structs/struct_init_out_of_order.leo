
// TODO: This should be made into an integration test.

struct Foo {
    a: u8,
    b: u16,
}

struct Bar {
    a: u8,
    b: u8,
}

program test.aleo {
    fn main(a: u8, b: u16) -> (Foo, Bar) {
        let c: u8 = a + a;
        let d: u16 = b * b;
        let e: Foo = Foo {
            b: d,
            a: c,
        };
        let f: Bar = Bar {
            b: c,
            a: a,
        };
        return (e, f);
    }
}




