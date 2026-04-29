interface Logger {
    storage counter: u64;
    storage entries: [u64];
}

program logger.aleo: Logger {
    storage counter: u64;
    storage entries: [u64];

    fn bump(val: u64) -> Final {
        return final {
            counter = counter.unwrap_or(0u64) + 1u64;
            entries.push(val);
        };
    }

    @noupgrade
    constructor() {}
}

// --- Next Program --- //
// ANCHOR: file
import logger.aleo;

program reader.aleo {
    mapping latest: u32 => u64;

    fn snapshot(target: field, i: u32) -> Final {
        return final { do_snapshot(target, i); };
    }

    @noupgrade
    constructor() {}
}

final fn do_snapshot(target: field, i: u32) {
    let n: u32 = logger.aleo::Logger@(target)::entries.len();
    let entry: u64? = logger.aleo::Logger@(target)::entries.get(i);
    let current: u64? = logger.aleo::Logger@(target)::counter;
    let stored: u64 = i < n ? entry.unwrap() : current.unwrap_or(0u64);
    Mapping::set(latest, i, stored);
}
// ANCHOR_END: file
