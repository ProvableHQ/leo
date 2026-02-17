program b28676.aleo {
    fn main() {
        let a = [1u32; 111111111111111u8]; // overflow
        let b = [1u32; 111111111111111]; // overflow
        let c = [1u32; 1u128]; // okay
        let d = [1u32; 513u128]; // passes limits
        let e = [1u32; 513]; // passes limits
    }
}
