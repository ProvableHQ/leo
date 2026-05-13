// ANCHOR: file
program first_public_state.aleo {
    mapping accumulator: u8 => u64;
    storage count: u8;
    storage queue: [u8];

    //=============================================================
    //               MAPPING MODIFICATION
    //=============================================================
    fn increment_accumulator() -> Final {
        return final {
            let current_count: u64 = accumulator.get_or_use(0u8, 0u64); // Get current value, defaults to 0
            let new_count: u64 = current_count + 1u64;
            accumulator.set(0u8, new_count);
        };
    }

    //=============================================================
    //            STORAGE VARIABLE MODIFICATION
    //=============================================================
    fn increment_count() -> Final {
        return final {
            let current_count: u8 = count.unwrap_or(0u8); // Get current value, defaults to 0
            count = current_count + 1u8;
        };
    }

    //=============================================================
    //            STORAGE VECTOR MODIFICATION
    //=============================================================
    fn add_to_queue(val: u8) -> Final {
        return final {
            queue.push(val); // Push to end of queue
        };
    }

    @noupgrade
    constructor() {}
}
// ANCHOR_END: file
