program test.aleo {
    const M: u32 = 52;

    struct Foo::[N: u32] {
        arr: [u32; N],
    }

    struct Goo::[P: u32] {
        f: Foo::[P],
    }

    struct Boo::[P: u32] {
        g: Goo::[P],
    }

    struct Baz {
        f: Foo::[10u32],
    }

    transition main(g: Goo::[7u32], b: Baz, x: u32) -> u32 {
        let g2 = Goo::[6u32] {
            f: Foo::[6u32] {
                arr: [42u32; 6],
            }
        };

        let s: Foo::[8u32] = Foo::[8u32] {
            arr: [3; 8]
        };

        let boo: Boo::[9u32] = Boo::[9u32] {
            g: Goo::[9u32] {
                f: Foo::[9u32] {
                    arr: [42u32; 9],
                }
            }
        };

        let baz = Baz {
            f: Foo::[10u32] {
                arr: [52; 10]
            }
        };

        return g.f.arr[6] + s.arr[7] + g2.f.arr[5] + boo.g.f.arr[8];
    }
}
