fn foo::[M: u32](arr: [u32; 3]) -> u32 {
    return arr[M - 1];
}

program test.aleo {
    fn main() -> u32 {
        return foo::[3u32]([1, 2, 3]);
    }
}
