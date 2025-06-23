program test.aleo {
    struct Foo::[N: u32] { y: u32 }

    transition main(x: u32) {
        let f = Foo::[x] { y: 0 };
    }
}
