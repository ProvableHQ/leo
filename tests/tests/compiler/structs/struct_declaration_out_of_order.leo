export struct F {
    b: B,
    g: G,
}

export struct B {
    a: A,
    d: D,
}

export struct A {
    data: u8,
}

export struct D {
    c: C,
    e: E,
}

export struct C {
    data: u8,
}

export struct E {
    data: u8,
}

export struct G {
    i: I,
}

export struct I {
    h: H,
}

export struct H {
    data: u8,
}

program test.aleo {
    fn foo(f: F) -> u8 {
        let a: u8 = f.b.a.data;
        let c: u8 = f.b.d.c.data;
        let e: u8 = f.b.d.e.data;
        let h: u8 = f.g.i.h.data;
        return a + c + e + h;
    }

    @noupgrade
    constructor() {}
}
