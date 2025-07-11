program test.aleo {

    mapping expected_checksum: bool => [u8; 32];

    transition foo() {}

    @checksum(mapping = "test.aleo/expected_checksum", key = "1field")
    async constructor() {}

}
