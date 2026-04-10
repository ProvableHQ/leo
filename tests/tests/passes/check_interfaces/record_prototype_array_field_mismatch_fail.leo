// Test: interface record prototype specifies a field with array type [u8; 32], but
// the implementing program provides [u8; 16]. The field types differ in array length,
// so this should fail with a record_field_type_mismatch error.

interface Vault {
    record Entry {
        owner: address,
        data: [u8; 32],
        ..
    }
    fn deposit(e: Entry) -> u64;
}

program test.aleo: Vault {
    record Entry {
        owner: address,
        data: [u8; 16],
    }

    fn deposit(e: Entry) -> u64 {
        return 0u64;
    }
}
