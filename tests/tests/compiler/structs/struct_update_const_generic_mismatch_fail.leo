struct Buffer::[N: u32] {
    data: [u32; N],
    len: u32,
}

program test.aleo {
    fn main(base: Buffer::[4], new_len: u32) -> u32 {
        // The base is a `Buffer::[4]` but the initializer is a `Buffer::[8]`, so the copied `data`
        // field would be an `[u32; 4]` filling an `[u32; 8]` slot. The mismatch must be rejected.
        let updated: Buffer::[8] = Buffer::[8] { len: new_len, ..base };
        return updated.len;
    }

    @noupgrade
    constructor() {}
}
