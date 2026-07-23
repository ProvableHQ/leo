program test.aleo {
    mapping expected: bool => [u8; 32];
    mapping admin: bool => address;

    fn set_expected(checksum: [u8; 32]) -> Final {
        let caller = std::ctx::caller();
        return final { set_expected_(caller, checksum); };
    }

    @custom
    constructor() {
        let current_edition: u16 = std::ctx::edition();
        if current_edition > 0u16 {
            let expected_checksum: [u8; 32] = expected.get(true);
            let current_checksum: [u8; 32] = std::ctx::checksum();
            assert_eq(expected_checksum, current_checksum);
        } else {
            admin.set(true, std::ctx::program_owner());
        }
    }
}

export final fn set_expected_(addr: address, checksum: [u8; 32]) {
    assert_eq(addr, admin.get(true));
    expected.set(true, checksum);
}
