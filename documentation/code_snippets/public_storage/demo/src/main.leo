// Compilable scaffold for `documentation/language/programs_in_practice/public_storage.md`.
// Each anchored region maps to one inline snippet in the doc.

program public_state_demo.aleo {

    // ANCHOR: mapping_decl
    mapping balance: address => u64;
    // ANCHOR_END: mapping_decl

    // ANCHOR: storage_var_decl
    storage counter: u64;
    // ANCHOR_END: storage_var_decl

    // ANCHOR: storage_vec_decl
    storage id_numbers: [u64];
    // ANCHOR_END: storage_vec_decl

    // ANCHOR: mapping_usage
    fn dubble() -> Final {
        let addr: address = self.caller;
        return final {
            let current_value: u64 = balance.get_or_use(addr, 0u64);
            balance.set(addr, current_value + 1u64);

            let next_current_value: u64 = balance.get(addr);
            balance.set(addr, next_current_value + 1u64);
        };
    }
    // ANCHOR_END: mapping_usage

    fn mapping_ops(addr: address, value: u64, fallback_value: u64) -> Final {
        return final {
            // ANCHOR: mapping_contains
            balance.contains(addr);
            Mapping::contains(balance, addr); // Alternate syntax
            // ANCHOR_END: mapping_contains

            // ANCHOR: mapping_get
            balance.get(addr);
            Mapping::get(balance, addr); // Alternate syntax
            // ANCHOR_END: mapping_get

            // ANCHOR: mapping_get_or_use
            balance.get_or_use(addr, fallback_value);
            Mapping::get_or_use(balance, addr, fallback_value); // Alternate syntax
            // ANCHOR_END: mapping_get_or_use

            // ANCHOR: mapping_set
            balance.set(addr, value);
            Mapping::set(balance, addr, value); // Alternate syntax
            // ANCHOR_END: mapping_set

            // ANCHOR: mapping_remove
            balance.remove(addr);
            Mapping::remove(balance, addr); // Alternate syntax
            // ANCHOR_END: mapping_remove
        };
    }

    // ANCHOR: storage_var_usage
    fn increment() -> Final {
        return final {
            let current_value: u64 = counter.unwrap_or(0u64);
            counter = current_value + 1u64;
        };
    }
    // ANCHOR_END: storage_var_usage

    fn storage_var_ops(fallback_value: u64) -> Final {
        return final {
            // ANCHOR: storage_var_unwrap
            let cur1: u64 = counter.unwrap();
            // ANCHOR_END: storage_var_unwrap

            // ANCHOR: storage_var_unwrap_or
            let cur2: u64 = counter.unwrap_or(fallback_value);
            // ANCHOR_END: storage_var_unwrap_or

            // ANCHOR: storage_var_set
            counter = 5u64;
            // ANCHOR_END: storage_var_set

            // ANCHOR: storage_var_unset
            counter = none;
            // ANCHOR_END: storage_var_unset
        };
    }

    // ANCHOR: storage_vec_usage
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
    // ANCHOR_END: storage_vec_usage

    fn storage_vec_ops(idx: u32, value: u64) -> Final {
        return final {
            // ANCHOR: storage_vec_get
            id_numbers.get(idx);
            // ANCHOR_END: storage_vec_get

            // ANCHOR: storage_vec_len
            id_numbers.len();
            // ANCHOR_END: storage_vec_len

            // ANCHOR: storage_vec_set
            id_numbers.set(idx, value);
            // ANCHOR_END: storage_vec_set

            // ANCHOR: storage_vec_push
            id_numbers.push(value);
            // ANCHOR_END: storage_vec_push

            // ANCHOR: storage_vec_pop
            id_numbers.pop();
            // ANCHOR_END: storage_vec_pop

            // ANCHOR: storage_vec_swap_remove
            id_numbers.swap_remove(idx);
            // ANCHOR_END: storage_vec_swap_remove

            // ANCHOR: storage_vec_clear
            id_numbers.clear();
            // ANCHOR_END: storage_vec_clear
        };
    }

    @noupgrade
    constructor() {}
}
