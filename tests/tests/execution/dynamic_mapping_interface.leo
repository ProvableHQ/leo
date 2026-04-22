/*
seed = 123456789
start_height = 20

[case]
program = "bank.aleo"
function = "deposit"
input = ["1u64", "42u64"]

[case]
program = "bank.aleo"
function = "check_contains"
input = ["1802396002field", "1u64"]

[case]
program = "bank.aleo"
function = "check_get"
input = ["1802396002field", "1u64", "42u64"]

[case]
program = "bank.aleo"
function = "get_or_use_existing"
input = ["1802396002field", "1u64", "0u64"]

[case]
program = "bank.aleo"
function = "get_or_use_missing"
input = ["1802396002field", "999u64", "0u64"]
*/

// Field encoding: bank = 1802396002field

interface Bank {
    mapping balances: u64 => u64;
}

program bank.aleo: Bank {
    mapping balances: u64 => u64;

    // Populate the mapping with key->value.
    fn deposit(key: u64, val: u64) -> Final {
        return final { finalize_deposit(key, val); };
    }

    // Verify that contains.dynamic finds the key.
    fn check_contains(prog: field, key: u64) -> Final {
        return final { fin_check_contains(prog, key); };
    }

    // Verify that get.dynamic retrieves the correct value.
    fn check_get(prog: field, key: u64, expected: u64) -> Final {
        return final { fin_check_get(prog, key, expected); };
    }

    // Verify get.or_use.dynamic returns the stored value for an existing key.
    fn get_or_use_existing(prog: field, key: u64, default_val: u64) -> Final {
        return final { fin_get_or_use_existing(prog, key, default_val); };
    }

    // Verify get.or_use.dynamic returns the default for a missing key.
    fn get_or_use_missing(prog: field, key: u64, default_val: u64) -> Final {
        return final { fin_get_or_use_missing(prog, key, default_val); };
    }

    @noupgrade
    constructor() {}
}

final fn finalize_deposit(key: u64, val: u64) {
    Mapping::set(balances, key, val);
}

final fn fin_check_contains(prog: field, key: u64) {
    let exists: bool = Bank@(prog)::balances.contains(key);
    assert(exists);
}

final fn fin_check_get(prog: field, key: u64, expected: u64) {
    let val: u64 = Bank@(prog)::balances.get(key);
    assert_eq(val, expected);
}

final fn fin_get_or_use_existing(prog: field, key: u64, default_val: u64) {
    // key 1u64 is in the mapping with value 42u64.
    let val: u64 = Bank@(prog)::balances.get_or_use(key, default_val);
    assert_eq(val, 42u64);
}

final fn fin_get_or_use_missing(prog: field, key: u64, default_val: u64) {
    // key 999u64 is absent; expect the default 0u64.
    let val: u64 = Bank@(prog)::balances.get_or_use(key, default_val);
    assert_eq(val, default_val);
}
