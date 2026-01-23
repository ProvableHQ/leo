program test.aleo {

    mapping expected_checksum: bool => [u8; 32];

    fn foo() {}

    @checksum(mapping = "test.aleo/expected_checksum", key = "true")
    constructor() {}
}
