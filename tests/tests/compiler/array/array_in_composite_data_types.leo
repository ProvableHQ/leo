
struct bar {
    data: [u8; 8],
}

program test.aleo {
    record floo {
        owner: address,
        data: [u8; 8],
    }

    fn foo(a: [[bool; 8]; 8]) -> (bool, bar, floo) {
        return (a[0u32][0u32], bar { data: [1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8] }, floo { owner: self.signer, data: [1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8] });
    }
}
