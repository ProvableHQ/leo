program negative_storage_tests.aleo {
    record MyRecord {
        owner: address
    }

    // 1. Illegal storage types
    storage tuple_storage: (u32, u32);
    storage future_storage: Future;
    storage address_storage: address;
    storage signature_storage: signature;
    storage record_storage: MyRecord;
    storage unit_storage: ();

    // 2. Storage vectors containing illegal types
    storage vec_tuple: [(u32, u32)];
    storage vec_future: [Future];
    storage vec_address: [address];
    storage vec_signature: [signature];
    storage vec_record: [MyRecord];
    storage vec_unit: [()];

    transition foo() {}
}
