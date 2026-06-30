// Pseudo-random value generation in the finalize context.
//
// Every function in this module produces a fresh value drawn from a ChaCha
// stream cipher whose seed is derived from the current block's pre-finalize
// state, the executing transition's identifier, and a per-call nonce. The
// randomness is **deterministic across re-executions of the same
// transition**: every validator finalizing the same transition observes
// the same sequence of values, so all validators reach consensus on the
// result. Distinct transitions, even when they run the same finalize
// logic against the same inputs, see independent streams.
//
// All functions here are `final fn`s and can only be called from inside a
// `final fn` body or a `final { ... }` async block. They are appropriate
// for sampling lottery winners, jittering reward schedules, or any other
// on-chain randomness that doesn't need to be unpredictable by the block
// proposer. (A miner can observe the seed and selectively withhold
// or reorder transactions.) For randomness that must resist a malicious
// proposer, use a commit-reveal scheme on top of these primitives.

// Returns a uniformly-random Aleo address.
export final fn chacha_address() -> address {
    return _chacha_rand_address();
}

// Returns a uniformly-random boolean.
export final fn chacha_bool() -> bool {
    return _chacha_rand_bool();
}

// Returns a uniformly-random field element.
export final fn chacha_field() -> field {
    return _chacha_rand_field();
}

// Returns a uniformly-random group element on Aleo's curve.
export final fn chacha_group() -> group {
    return _chacha_rand_group();
}

// Returns a uniformly-random scalar (an element of the curve's scalar field).
export final fn chacha_scalar() -> scalar {
    return _chacha_rand_scalar();
}

// Returns a uniformly-random `u8`.
export final fn chacha_u8() -> u8 {
    return _chacha_rand_u8();
}

// Returns a uniformly-random `u16`.
export final fn chacha_u16() -> u16 {
    return _chacha_rand_u16();
}

// Returns a uniformly-random `u32`.
export final fn chacha_u32() -> u32 {
    return _chacha_rand_u32();
}

// Returns a uniformly-random `u64`.
export final fn chacha_u64() -> u64 {
    return _chacha_rand_u64();
}

// Returns a uniformly-random `u128`.
export final fn chacha_u128() -> u128 {
    return _chacha_rand_u128();
}

// Returns a uniformly-random `i8`.
export final fn chacha_i8() -> i8 {
    return _chacha_rand_i8();
}

// Returns a uniformly-random `i16`.
export final fn chacha_i16() -> i16 {
    return _chacha_rand_i16();
}

// Returns a uniformly-random `i32`.
export final fn chacha_i32() -> i32 {
    return _chacha_rand_i32();
}

// Returns a uniformly-random `i64`.
export final fn chacha_i64() -> i64 {
    return _chacha_rand_i64();
}

// Returns a uniformly-random `i128`.
export final fn chacha_i128() -> i128 {
    return _chacha_rand_i128();
}
