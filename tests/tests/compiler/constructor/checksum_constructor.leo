program test.aleo {

    mapping expected: bool => [u8; u32];

    transition foo() {}

    async constructor() {
        if self.edition > 0u16 {
            let expected: [u8; 32] = expected.get(true);
            assert_eq(self.checksum, expected);
        }
    }
}
