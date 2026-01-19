const N: u32 = 2u32;

program b28676.aleo {
    fn main() -> u32 {
        for i in 0u32..5u32 {
            let arr: [u32; i] = [1u32, 2u32]; // this should fail for all iterations except 2
        }
        return 0u32;
    }
}
