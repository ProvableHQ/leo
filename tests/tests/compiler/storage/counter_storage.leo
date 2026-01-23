program test.aleo {
    storage counter: u32;

    fn initialize() -> Final {
        return async {
            counter = 0;
        };
    }

    fn increment() -> Final {
        return async {
            let old_counter_value = counter.unwrap_or(0);
            counter = old_counter_value + 1u32;
            assert(counter.unwrap() == old_counter_value + 1u32);
        };
    }

    @noupgrade
    constructor() {}
}
