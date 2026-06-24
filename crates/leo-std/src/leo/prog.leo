// On-chain metadata accessors for imported programs.
//
// Each function in this module reads finalize-store state of a *named*
// program — the program identifier is a const generic argument, so it is
// fixed at compile time. Use these to gate logic on the on-chain version,
// checksum, or ownership of a dependency program.

// Returns the 32-byte deployment checksum of the program identified by
// `PROG`. The checksum changes only when the program is upgraded with
// new bytecode, so two programs with the same source and dependencies
// will agree on this value.
@_program_id_arg
final fn checksum::[PROG: address]() -> [u8; 32] {
    return _program_checksum(PROG);
}

// Returns the deployment edition of the program identified by `PROG`.
// Edition `0` is the initial deployment; every upgrade increments it.
// Useful for guarding logic against specific upgrade versions of a
// dependency.
@_program_id_arg
final fn edition::[PROG: address]() -> u16 {
    return _program_edition(PROG);
}

// Returns the address that owns the program identified by `PROG` —
// typically the account that performed the deployment.
//
// Programs deployed before the upgradability feature shipped do not have
// an owner; reading this value on such a program halts at runtime.
@_program_id_arg
final fn program_owner::[PROG: address]() -> address {
    return _program_owner(PROG);
}

// Returns the 32-byte checksum of a specific function (`FN_NAME`) inside
// the program identified by `PROG`. Useful for asserting that a particular
// function of a dependency program has not changed between upgrades.
@_program_id_arg
@_callable_function_arg
final fn function_checksum::[PROG: address, FN_NAME: identifier]() -> [u8; 32] {
    return _function_checksum(PROG, FN_NAME);
}
