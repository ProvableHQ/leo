program storage_vector.aleo {
    storage id_numbers: [u64];

    fn add_id(new_id: u64) -> Final {
        return final {
            id_numbers.push(new_id);
        };
    }

    fn remove_id(idx: u32) -> Final {
        return final {
            id_numbers.swap_remove(idx);
        };
    }

    @noupgrade
    constructor() {}
}
