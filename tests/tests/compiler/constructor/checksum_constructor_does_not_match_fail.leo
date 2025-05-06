/*
upgrade = { "mode": "checksum", "mapping": "expected_checksum", "key": "true" } 
*/

program test.aleo {

    mapping expected_checksum: bool => [u8; 32];

    transition foo() {}

    async constructor() {
        if self.edition >= 0u16 {
            let expected: [u8; 32] = Mapping::get(expected_checksum, true);
            assert_eq(self.checksum, expected);
        }
    }
}
