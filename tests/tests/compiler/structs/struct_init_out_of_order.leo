/*
namespace: Compile
expectation: Pass
*/

// TODO: This should be made into an integration test.

program test.aleo {
    struct Foo {
        a: u8,
        b: u16,
    }

    struct Bar {
        a: u8,
        b: u8,
    }

    transition main(a: u8, b: u16) -> (Foo, Bar) {
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




