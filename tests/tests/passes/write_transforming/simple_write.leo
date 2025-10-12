program test.aleo {
    transition main(x: u32, y: u32) -> u32 {
        let a: u32 = x + y;
        let b: u32 = a * 2u32;
        return b;
    }
}

