// The 'test_json_program' program.
program test_json_program.aleo {
    mapping expected_checksum: bool => [u8; 32];

    fn main(public a: u32, b: u32) -> u32 {
        let c: u32 = a + b;
        return c;
    }

    @checksum(mapping = "test_json_program.aleo/expected_checksum", key = "true")
    constructor() {}
}

