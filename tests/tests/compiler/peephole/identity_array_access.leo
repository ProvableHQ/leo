program test.aleo {
    fn main(a: [u32; 2]) -> u32 {
        let y: u32 = a[0u32] + 0u32;
        return y;
    }

    @noupgrade
    constructor() {}
}
