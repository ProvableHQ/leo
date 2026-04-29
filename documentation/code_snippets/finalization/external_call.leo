program first_public_storage.aleo {
    storage count: u8;

    fn increment() -> Final {
        return final {
            count = count.unwrap_or(0u8) + 1u8;
        };
    }

    @noupgrade
    constructor() {}
}

// --- Next Program --- //
// ANCHOR: file
import first_public_storage.aleo;

program second_public_storage.aleo {
    mapping hashes: u8 => scalar;

    fn two_mappings(value: u8) -> Final {
        let increment_final: Final = first_public_storage.aleo::increment();
        return final {
            increment_final.run();
            let hash: scalar = BHP256::hash_to_scalar(value);
            hashes.set(value, hash);
        };
    }

    @noupgrade
    constructor() {}
}
// ANCHOR_END: file
