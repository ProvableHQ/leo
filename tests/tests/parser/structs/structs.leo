struct Foo1 {
    y: u8
}
struct Foo2::[N: u32] {
    y: u8
}
struct Foo3::[N: u32, M: address, N: signature] {
    z: Foo2::[5],
}

program comma_separators.aleo {
    fn main() {
    }
}
