interface AsyncTarget {
    fn execute(value: u32) -> Final;
}

program producer.aleo: AsyncTarget {
    mapping values: u32 => u32;

    fn execute(value: u32) -> Final {
        return final {
            Mapping::set(values, value, value);
        };
    }

    fn route(target: field, value: u32) -> Final {
        let future: Final = AsyncTarget@(target)::execute(value);
        return final {
            future.run();
        };
    }

    @noupgrade
    constructor() {}
}

// --- Next Program --- //

import producer.aleo;

program consumer.aleo {
    fn forward(target: field, value: u32) -> Final {
        let future: Final = producer.aleo::route(target, value);
        return final {
            future.run();
        };
    }

    @noupgrade
    constructor() {}
}
