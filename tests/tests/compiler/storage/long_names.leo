program st.aleo {
    // Both of these names are 31 chars long which is the limit.
    // When converted to mappings, their names get longer. Ensure that codegen
    // can handle converting long names correctly.
    storage some_very_long_storage_variable: u8;
    storage some_verrry_long_storage_vector: [u8];
    async transition foo() -> Future {
        return async {
            some_very_long_storage_variable = 0;
            some_verrry_long_storage_vector.push(some_very_long_storage_variable.unwrap());
        };
    }
}
