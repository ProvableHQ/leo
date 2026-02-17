struct Foo::[N: u32] { y: u32 }

program test.aleo {
    fn main(x: u32) {
        let f = Foo::[x] { y: 0 };
    }
}
