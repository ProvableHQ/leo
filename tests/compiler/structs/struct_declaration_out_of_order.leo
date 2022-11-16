/*
namespace: Compile
expectation: Pass
*/

program test.aleo {

    struct F {
        b: B,
        g: G,
    }

    struct B {
        a: A,
        d: D,
    }

    struct A {
        data: u8,
    }

    struct D {
        c: C,
        e: E,
    }

    struct C {
        data: u8,
    }

    struct E {
        data: u8,
    }

    struct G {
        i: I,
    }

    struct I {
        h: H,
    }

    struct H {
        data: u8,
    }

    transition foo(f: F) -> u8 {
        let a: u8 = f.b.a.data;
        let c: u8 = f.b.d.c.data;
        let e: u8 = f.b.d.e.data;
        let h: u8 = f.g.i.h.data;
        return a + c + e + h;
    }
}
