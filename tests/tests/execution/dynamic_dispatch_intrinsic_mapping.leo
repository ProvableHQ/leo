/*
seed = 123456789
start_height = 20

[case]
program = "dyn_mapping.aleo"
function = "set"
input = ["1u64", "42u64"]

[case]
program = "dyn_mapping.aleo"
function = "check_contains"
input = ["125040764743633057491089764field", "1868917857field", "109field", "1u64"]

[case]
program = "dyn_mapping.aleo"
function = "check_get"
input = ["125040764743633057491089764field", "1868917857field", "109field", "1u64"]

[case]
program = "dyn_mapping.aleo"
function = "get_use_existing"
input = ["125040764743633057491089764field", "1868917857field", "109field", "1u64"]

[case]
program = "dyn_mapping.aleo"
function = "get_use_missing"
input = ["125040764743633057491089764field", "1868917857field", "109field", "999u64"]
*/

// Field encodings used above:
//   prog  = 125040764743633057491089764field  -- "dyn_mapping" (identifier part of dyn_mapping.aleo)
//   net   = 1868917857field                  -- "aleo" (domain part)
//   map   = 109field                         -- "m" (mapping name)

program dyn_mapping.aleo {
    mapping m: u64 => u64;

    // Populate the mapping with key->value.
    fn set(key: u64, val: u64) -> Final {
        return final { finalize_set(key, val); };
    }

    // Verify that _dynamic_contains finds the key.
    fn check_contains(prog: field, net: field, map: field, key: u64) -> Final {
        return final { fin_check_contains(prog, net, map, key); };
    }

    // Verify that _dynamic_get retrieves the correct value.
    fn check_get(prog: field, net: field, map: field, key: u64) -> Final {
        return final { fin_check_get(prog, net, map, key); };
    }

    // Verify _dynamic_get_or_use returns the stored value for an existing key.
    fn get_use_existing(prog: field, net: field, map: field, key: u64) -> Final {
        return final { fin_get_use_existing(prog, net, map, key); };
    }

    // Verify _dynamic_get_or_use returns the default for a missing key.
    fn get_use_missing(prog: field, net: field, map: field, key: u64) -> Final {
        return final { fin_get_use_missing(prog, net, map, key); };
    }

    @noupgrade
    constructor() {}
}

final fn finalize_set(key: u64, val: u64) {
    Mapping::set(m, key, val);
}

final fn fin_check_contains(prog: field, net: field, map: field, key: u64) {
    let exists: bool = _dynamic_contains(prog, net, map, key);
    assert(exists);
}

final fn fin_check_get(prog: field, net: field, map: field, key: u64) {
    let val: u64 = _dynamic_get::[u64](prog, net, map, key);
    assert_eq(val, 42u64);
}

final fn fin_get_use_existing(prog: field, net: field, map: field, key: u64) {
    // key 1u64 is in the mapping with value 42u64.
    let val: u64 = _dynamic_get_or_use::[u64](prog, net, map, key, 0u64);
    assert_eq(val, 42u64);
}

final fn fin_get_use_missing(prog: field, net: field, map: field, key: u64) {
    // key 999u64 is absent; expect the default 0u64.
    let val: u64 = _dynamic_get_or_use::[u64](prog, net, map, key, 0u64);
    assert_eq(val, 0u64);
}
