program module_resolution_corner.aleo {
    struct Wrapper {
        outer: base::Foo,
        inner: base::nested::Foo::[3],
    }

    transition main() -> u32 {
        let outer_x = base::X;
        let inner_x = base::nested::X;
        let deep_x = base::nested::deeper::X;

        let f1 = base::Foo {
            a: [1; base::X],
        };

        let f2 = base::nested::Foo::[3] {
            a: [2; 3],
        };

        let f3 = base::nested::deeper::Foo::[base::nested::deeper::X] {
            a: [3; base::nested::deeper::X],
        };

        return outer_x + inner_x + deep_x
             + f1.a[0] + f2.a[1] + f3.a[2]
             + base::nested::compute()
             + base::nested::deeper::compute();
    }
}

// --- Next Module: base.leo --- //

const X: u32 = 2;

struct Foo {
    a: [u32; X],
}

// --- Next Module: base/nested.leo --- //

const X: u32 = 3;

struct Foo::[N: u32] {
    a: [u32; N],
}

inline compute() -> u32 {
    let b = Foo::[X] {
        a: [10; X],
    };
    return b.a[1] + deeper::X + deeper::compute();
}

// --- Next Module: base/nested/deeper.leo --- //

const X: u32 = 4;

struct Foo::[M: u32] {
    a: [u32; M],
}

inline compute() -> u32 {
    let f = Foo::[X] {
        a: [20; X],
    };
    return X + f.a[2];
}

