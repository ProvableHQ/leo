program test.aleo {
    fn main(x: u32, y: u32, z: u32) -> u32 {
        let a: u32 = x * y + z;
        let b: u32 = x * y + z;
        let c: u32 = x * y;
        return a + b + c;
    }
}
