program good_storage_tests.aleo {
    // --- Primitives ---
    storage bool_storage: bool;
    storage scalar_storage: scalar;
    storage field_storage: field;
    storage group_storage: group;
    storage address_storage: address;
    storage u32_storage: u32;
    storage i8_storage: i8;

    // --- Structs ---
    struct MyStruct {
        a: u32,
        b: bool,
        c: address,
    }
    storage struct_storage: MyStruct;

    // --- Arrays ---
    storage array_u32: [u32; 3];
    storage array_bool: [bool; 2];
    storage array_address: [address; 2];
    storage array_struct: [MyStruct; 2];

    // --- Vectors ---
    storage vec_bool: [bool];
    storage vec_scalar: [scalar];
    storage vec_field: [field];
    storage vec_group: [group];
    storage vec_address: [address];
    storage vec_u32: [u32];
    storage vec_i8: [i8];
    storage vec_struct: [MyStruct];

    transition foo() {}
}

