/*
seed = 123456789
start_height = 20

[case]
program = "bank_impl.aleo"
function = "deposit"
input = ["1u64", "42u64"]

[case]
program = "checker.aleo"
function = "check_contains"
input = ["2000349607123886170466field", "1u64"]

[case]
program = "checker.aleo"
function = "check_get"
input = ["2000349607123886170466field", "1u64", "42u64"]

[case]
program = "checker.aleo"
function = "check_get_or_use_missing"
input = ["2000349607123886170466field", "999u64", "0u64"]
*/

// Field encodings:
//   bank_impl = 2000349607123886170466field
//   checker   = 32199659248511075field

interface Bank {
    mapping balances: u64 => u64;
}

program bank_impl.aleo: Bank {
    mapping balances: u64 => u64;

    fn deposit(key: u64, val: u64) -> Final {
        return final { finalize_deposit(key, val); };
    }

    @noupgrade
    constructor() {}
}

final fn finalize_deposit(key: u64, val: u64) {
    Mapping::set(balances, key, val);
}

// --- Next Program --- //
import bank_impl.aleo;

program checker.aleo {
    fn check_contains(target: field, key: u64) -> Final {
        return final { fin_check_contains(target, key); };
    }

    fn check_get(target: field, key: u64, expected: u64) -> Final {
        return final { fin_check_get(target, key, expected); };
    }

    fn check_get_or_use_missing(target: field, key: u64, default_val: u64) -> Final {
        return final { fin_check_get_or_use_missing(target, key, default_val); };
    }

    @noupgrade
    constructor() {}
}

final fn fin_check_contains(target: field, key: u64) {
    let exists: bool = bank_impl.aleo::Bank@(target)::balances.contains(key);
    assert(exists);
}

final fn fin_check_get(target: field, key: u64, expected: u64) {
    let val: u64 = bank_impl.aleo::Bank@(target)::balances.get(key);
    assert_eq(val, expected);
}

final fn fin_check_get_or_use_missing(target: field, key: u64, default_val: u64) {
    // key 999u64 is absent; expect the default.
    let val: u64 = bank_impl.aleo::Bank@(target)::balances.get_or_use(key, default_val);
    assert_eq(val, default_val);
}
