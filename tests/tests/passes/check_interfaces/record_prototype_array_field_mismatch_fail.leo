// Test: interface record prototype specifies a field with array type [u8; 32], but
// the implementing program provides [u8; 16]. Should produce record_field_type_mismatch.
// This verifies that visit_record_prototype's member-type visitation makes field type
// information available for validation — not just for preventing const-prop panics.

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
