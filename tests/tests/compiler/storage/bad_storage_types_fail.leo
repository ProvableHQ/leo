program negative_storage_tests.aleo {
    // --- Top-level illegal types ---
    storage unit_storage: ();
    storage string_storage: string;
    storage signature_storage: signature;
    storage future_storage: Final;
    storage optional_storage: u32?;
    storage tuple_storage: (u32, u32);

    // --- Record (disallowed composite) ---
    record MyRecord {
        owner: address
    }
    storage record_storage: MyRecord;

    // --- Arrays of illegal types ---
    storage array_unit: [(); 2];
    storage array_string: [string; 2];
    storage array_signature: [signature; 2];
    storage array_future: [Final; 2];
    storage array_optional: [u32?; 2];
    storage array_tuple: [(u32, u32); 2];
    storage array_record: [MyRecord; 2];

    // --- Vectors of illegal types ---
    storage vec_unit: [()];
    storage vec_string: [string];
    storage vec_signature: [signature];
    storage vec_future: [Final];
    storage vec_optional: [u32?];
    storage vec_tuple: [(u32, u32)];
    storage vec_record: [MyRecord];

    fn foo() {}
}
