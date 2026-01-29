const B: (u8,u32) = (1u8,1u32);

program test.aleo {
    fn foo() -> u32 {
        for i: u32 in 0u32..1u32 {
            let d:(u8,u32) = B;
            const A:u32 = 1u32;
            let c:u32 = A;
        }
        return 1u32;
    }
}
