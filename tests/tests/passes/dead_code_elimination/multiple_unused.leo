program test.aleo {
    transition main(x: u32) -> u32 {
        let unused1: u32 = 10u32;
        let result: u32 = x * 2u32;
        let unused2: u32 = 20u32;
        return result;
    }
}
