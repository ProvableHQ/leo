program test.aleo {
    transition main() -> u32 {
        let x: u32 = 5u32 + 3u32;
        let y: u32 = x * 2u32;
        return y;
    }
    transition foo() -> u32 {
        let x: u32 = 5u32 + 3u32;
        let y: u32 = x * 2u32;
        let z = (x, y);
        let (a, b) = z;
        return a + b;
    }
}

