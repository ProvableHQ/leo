program test.aleo {
    // Regression: an array written element-wise then used in a repeat `[arr; N]` must be rebuilt from its writes.
    fn write_then_repeat(public p0: u8, public p1: u8) -> public [[u8; 2]; 3] {
        let arr: [u8; 2] = [0u8; 2];
        for i: u32 in 0u32..2u32 {
            arr[i] = p0 + p1 + (i as u8);
        }
        let result: [[u8; 2]; 3] = [arr; 3u32];
        return result;
    }

    @noupgrade
    constructor() {}
}
