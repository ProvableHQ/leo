program b28676.aleo {
    fn main() {
        let a: [u32; 111111111111111u8] = [1u32]; // overflow
        let b: [u32; 111111111111111] = [1u32]; // overflow
        let c: [u32; 1u128] = [1u32]; // okay
        let d: [u32; 513u128] = [1u32]; // passes limits
        let e: [u32; 513] = [1u32]; // passes limits
    }
}
