// Execution-context accessors for the current transition.
//
// Functions in this module surface information about the program that is
// executing right now: who called it, which signer authorized the outer
// transaction, what the current block height and timestamp are, and so on.

// Returns the Aleo program address of this program.
fn addr() -> address {
    return _self_address();
}

// Returns the immediate caller of this transition. If the transition was
// invoked directly by an external user, the caller equals the signer; if it
// was invoked by another program, the caller is that program's address.
//
// Use `caller` for trust decisions about who is asking for the current
// operation. Use `signer` when the decision should track the
// originator of the entire transaction.
fn caller() -> address {
    return _self_caller();
}

// Returns the address that signed the outer transaction. Unlike `caller`,
// this never changes during cross-program calls: it always points to the
// off-chain agent who authorized the whole call graph.
fn signer() -> address {
    return _self_signer();
}

// Returns the on-chain identifier of this program. The id is an Aleo address
// derived from the program's source and is the same value users see in block
// explorers.
final fn id() -> address {
    return _self_id();
}

// Returns the 32-byte deployment checksum of this program. The checksum is
// fixed at deployment time and changes if (and only if) the program is
// upgraded with new bytecode.
final fn checksum() -> [u8; 32] {
    return _self_checksum();
}

// Returns the deployment edition of this program. Edition `0` is the initial
// deployment; each upgrade increments the edition by one.
final fn edition() -> u16 {
    return _self_edition();
}

// Returns the address that owns this program, typically the account that
// performed the deployment. Owner-only on-chain operations should compare
// `signer()` against `program_owner()`.
final fn program_owner() -> address {
    return _self_program_owner();
}

// Returns the height of the block containing the current transaction. Block
// height increases by one with every finalized block on the network.
final fn block_height() -> u32 {
    return _block_height();
}

// Returns the Unix timestamp (in seconds) of the block containing the current
// transaction. The value reflects the block producer's clock at the time the
// block was proposed.
final fn block_timestamp() -> i64 {
    return _block_timestamp();
}

// Returns the numeric identifier of the network this program is executing on
// (e.g. mainnet, testnet, canary). Use this to gate behavior that differs
// between deployment targets.
final fn network_id() -> u16 {
    return _network_id();
}
