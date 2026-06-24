program test.aleo {
    mapping admin: bool => address;

    fn do_something(val: u64) -> Final {
        return final { do_something_(val); };
    }

    @custom
    constructor() {
        let current_edition: u16 = std::ctx::edition();
        if current_edition > 0u16 {
            let current_checksum: [u8; 32] = std::ctx::checksum();
        } else {
            admin.set(true, std::ctx::program_owner());
        }
    }
}

final fn do_something_(val: u64) {
    if val > 0u64 {
        assert(true);
    }
    if val > 1u64 {
        assert(true);
    }
}
