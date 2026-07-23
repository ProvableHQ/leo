struct Buffer::[N: u32] {
    data: [u32; N],
    len: u32,
}

program test.aleo {
    fn main(base: Buffer::[4], new_len: u32) -> u32 {
        // The missing `data` field (an `[u32; N]`) is copied from `base`.
        let updated: Buffer::[4] = Buffer::[4] { len: new_len, ..base };
        return updated.data[0u32] + updated.len;
    }

    @noupgrade
    constructor() {}
}
