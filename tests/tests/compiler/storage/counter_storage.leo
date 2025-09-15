program test.aleo {
    storage counter: u32;

    async transition initialize() -> Future {
        return async {
            counter = 0;
        };
    }

    async transition increment() -> Future {
        return async {
            let old_counter_value = counter.unwrap_or(0);
            counter = old_counter_value + 1u32;
            assert(counter.unwrap() == old_counter_value + 1u32);
        };
    }

    @noupgrade
    async constructor() {}
}
