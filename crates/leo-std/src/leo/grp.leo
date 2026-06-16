// Group operations on the Aleo curve.
//
// Aleo's group is an elliptic curve whose elements support addition,
// scalar multiplication, and conversion to and from affine `(x, y)`
// coordinates over the base field. This module exposes the two well-known
// generators in use throughout the protocol, the precomputed table of
// `H` powers, and coordinate extraction.
//
// Group elements are first-class Leo values of type `group`; the operations
// on them (`+`, `*` by a `scalar`, `.double()`, `.neg()`, `==`, ...) are
// available via Leo's built-in syntax and don't need to be wrapped here.

// The Ed25519 base point `G`, the canonical generator of the curve's
// prime-order subgroup. Use this to construct group elements from scratch
// or when implementing protocols that scalar-multiply a known generator.
fn generator() -> group {
    return _group_gen();
}

// The Aleo account-key generator `H`. Account addresses are derived as
// `pk * H` where `pk` is the secret view key; `H` is fixed by the protocol
// and resolved at runtime, so it cannot be folded at proving time.
fn aleo_generator() -> group {
    return _aleo_generator();
}

// The precomputed table `[H, 2H, 4H, 8H, ..., 2^250 * H]` used by Aleo's
// account-derivation routines. Useful when implementing custom signature
// or commitment schemes that mirror Aleo's own.
fn aleo_generator_powers() -> [group; 251] {
    return _aleo_generator_powers();
}

// Returns the affine `x` coordinate of `g` as a field element. Together with
// `to_y_coordinate`, this lets callers serialize a group element to a pair
// of field elements (e.g. for hashing into a commitment).
fn to_x_coordinate(g: group) -> field {
    return g.to_x_coordinate();
}

// Returns the affine `y` coordinate of `g` as a field element. See
// `to_x_coordinate` for context.
fn to_y_coordinate(g: group) -> field {
    return g.to_y_coordinate();
}
