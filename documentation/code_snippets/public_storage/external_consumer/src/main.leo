import external_program.aleo;

program consumer.aleo {
    fn read_var() -> Final {
        return final {
            // ANCHOR: ext_var_unwrap
            let value: u64 = external_program.aleo::counter.unwrap();
            // ANCHOR_END: ext_var_unwrap

            // ANCHOR: ext_var_unwrap_or
            let value2: u64 = external_program.aleo::counter.unwrap_or(0u64);
            // ANCHOR_END: ext_var_unwrap_or
        };
    }

    fn read_vec() -> Final {
        return final {
            // ANCHOR: ext_vec_reads
            let first: u64? = external_program.aleo::id_numbers.get(0u32);
            let length: u32 = external_program.aleo::id_numbers.len();
            // ANCHOR_END: ext_vec_reads
        };
    }

    @noupgrade
    constructor() {}
}
