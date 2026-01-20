program test.aleo {
    fn main() -> u32 {
        return foo::[2u32]([2, 3]);
    }
}
fn foo::[M: u32](arr: [u32; M]) -> u32 {
    return arr[20];
}
