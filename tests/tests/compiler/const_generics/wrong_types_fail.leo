program test.aleo {
    struct Foo::[N: u32] { x: u32 }

    transition main(a: Foo::[5u16], b: Foo::[], c: Foo::[5u32, 5u32]) {
        let x = foo::[5u16]();
        let y = foo::[]();
        let z = foo::[5u32, 5u32]();

        let f1: Foo::[5u16] = Foo::[5u16] { x: 0 };
        let f2: Foo::[] = Foo::[] { x: 0 };
        let f3: Foo::[5u32, 5u32] = Foo::[5u32, 5u32] { x: 0 };

        let f4: Foo::[5] = Foo::[4] { x: 0 };
    }

    inline foo::[N: u32]() -> u32 {
        return N;
    }
}
