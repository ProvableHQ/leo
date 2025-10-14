program multifile.aleo {
    struct QQ {
        y: dep::Baz::[3],
    }

    transition main() -> u32 {
        let x = dep::X;
        let inner_x = inner::dep::X;
        let f = dep::Foo::[dep::X] {
            a: [42; dep::X],
        };
        let inner_f = inner::dep::Foo {
            a: [69; inner::dep::X],
        };
        return x + inner_x
              + f.a[dep::X / 2] + inner_f.a[inner::dep::X / 3]
              + dep::bar() + inner::dep::bar() + goo::[30]()
              + dep::goo::[31]() + dep::other_dep::Y;
    }

    inline goo::[M: u32]() -> u32 { 
        let b = dep::Bar::[M] {
            a: [43; M],
            c: dep::Baz::[M] {
                x: 169, 
            }
        };
        b.a[M/M] = 999999;
        return M + dep::X + b.a[M/2]; 
    }
}

// --- Next Module: dep.leo --- //

struct Baz::[Q: u32] {
    x: u32,
}

const X: u32 = 6;

struct Foo::[N: u32] {
    a: [u32; N],
}


struct Bar::[N: u32] {
    a: [u32; N],
    c: Baz::[N],
}

inline bar() -> u32 {
    let c = Foo::[X] {
        a: [66; X]
    };
    let b:[u32; 3] = [1, 2, 3];
    b[2] = 4;
    c.a[0] = 1000000 + b[2];
    return c.a[0] + goo::[29]() + X + other_dep::Y;
}

inline goo::[M: u32]() -> u32 { 
    let c = Baz::[M] {
        x: 69,
    };
    let b = Bar::[M] {
        a: [43; M],
        c,
    };
    b.a[M/2] = 333333;
    return M + X + b.a[M/2]; 
}

// --- Next Module: dep/other_dep.leo --- //
const Y: u32 = 999999;

// --- Next Module: inner/dep.leo --- //
const X: u32 = 11;

struct Foo {
    a: [u32; X],
}

struct Bar::[N: u32] {
    a: [u32; N],
}

inline bar() -> u32 {
    let c = Foo {
        a: [6; X]
    };
    return c.a[0] + goo::[99]() + X;
}

inline goo::[N: u32]() -> u32 { return N + X; }
