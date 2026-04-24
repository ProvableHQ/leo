/*
seed = 123456789
start_height = 20

[case]
program = "logger.aleo"
function = "bump"
input = ["10u64"]

[case]
program = "logger.aleo"
function = "bump"
input = ["20u64"]

[case]
program = "logger.aleo"
function = "check_counter"
input = ["125779852095340field", "2u64"]

[case]
program = "logger.aleo"
function = "check_entry"
input = ["125779852095340field", "0u32", "10u64"]

[case]
program = "logger.aleo"
function = "check_entry"
input = ["125779852095340field", "1u32", "20u64"]

[case]
program = "logger.aleo"
function = "check_entry_oob"
input = ["125779852095340field", "99u32"]

[case]
program = "logger.aleo"
function = "check_len"
input = ["125779852095340field", "2u32"]
*/

// Field encoding: logger = 125779852095340field

interface Logger {
    storage counter: u64;
    storage entries: [u64];
}

program logger.aleo: Logger {
    storage counter: u64;
    storage entries: [u64];

    // Populate storage: bump the counter and append to the vector.
    fn bump(val: u64) -> Final {
        return final {
            counter = counter.unwrap_or(0u64) + 1u64;
            entries.push(val);
        };
    }

    // Read the singleton storage via dynamic dispatch and assert the unwrapped value.
    fn check_counter(target: field, expected: u64) -> Final {
        return final { fin_check_counter(target, expected); };
    }

    // Read a vector element via dynamic dispatch and assert it equals expected.
    fn check_entry(target: field, i: u32, expected: u64) -> Final {
        return final { fin_check_entry(target, i, expected); };
    }

    // Read an out-of-bounds vector element and assert the Option is None.
    fn check_entry_oob(target: field, i: u32) -> Final {
        return final { fin_check_entry_oob(target, i); };
    }

    // Read the vector length via dynamic dispatch and assert it equals expected.
    fn check_len(target: field, expected: u32) -> Final {
        return final { fin_check_len(target, expected); };
    }

    @noupgrade
    constructor() {}
}

final fn fin_check_counter(target: field, expected: u64) {
    let val: u64? = Logger@(target)::counter;
    assert_eq(val.unwrap(), expected);
}

final fn fin_check_entry(target: field, i: u32, expected: u64) {
    let val: u64? = Logger@(target)::entries.get(i);
    assert_eq(val.unwrap(), expected);
}

final fn fin_check_entry_oob(target: field, i: u32) {
    let val: u64? = Logger@(target)::entries.get(i);
    assert(val == none);
}

final fn fin_check_len(target: field, expected: u32) {
    let n: u32 = Logger@(target)::entries.len();
    assert_eq(n, expected);
}
