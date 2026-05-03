program basic_voting.aleo {
    mapping approved_checksum: bool => [u8; 32];

    fn approve(value: [u8; 32]) -> Final {
        return final {
            Mapping::set(approved_checksum, true, value);
        };
    }

    @noupgrade
    constructor() {}
}
