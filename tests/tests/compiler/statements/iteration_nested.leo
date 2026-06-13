program test.aleo {
    fn main(x: u32) -> u32 {
        let y: u32 = x;
        for _i: u32 in 0u32..2u32 {
            for _j: u32 in 0u32..2u32 {
                y = y + 1u32;
            }
        }
        return y;
    }

    @noupgrade
    constructor() {}
}
