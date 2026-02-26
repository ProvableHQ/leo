program test.aleo {
    fn main(x: u32, y: u32, z: u32) -> u32 {
        let a: [u32; 3u32] = [x, y, z];
        a[0u32] = 0u32;
        return a[1u32];
    }
}

