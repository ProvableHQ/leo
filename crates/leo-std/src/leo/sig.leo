// On-chain signature verification.
//
// This module exposes two families of signature verifiers, Aleo's native
// Schnorr scheme and the ECDSA scheme used by Ethereum and other EVM-style
// chains, so that finalize logic can authenticate messages signed by users
// or by external systems.
//
// All verifiers are `final fn`s: signatures are checked on-chain so that
// validators reach consensus on the result. Off-chain authentication should
// be implemented at the transition level using account secrets, not via
// these functions.
//
// Each verifier returns `true` if and only if the signature is
// cryptographically valid for the supplied message and signer. Higher-level
// concerns, replay protection, nonce tracking, expiration, signature
// uniqueness across user accounts, remain the caller's responsibility.

// Verifies that `sig` is a Schnorr signature of `message` issued by `signer`.
//
// - `sig` is an Aleo Schnorr signature, typically produced by an account
//   holder using their private signing key.
// - `signer` is the Aleo address whose signing key produced `sig`.
// - `message` is the value that was signed. This wrapper accepts a `field`;
//   callers signing other primitive types should hash the value into a
//   `field` first (e.g. via one of the `std::hash::bhp256::hash_*_to_field`
//   wrappers).
//
// Returns `true` iff the signature verifies under `signer`'s public key.
final fn verify_schnorr(sig: signature, signer: address, message: field) -> bool {
    return _signature_verify(sig, signer, message);
}

// Verifies an ECDSA signature against a 32-byte prehash and a 33-byte
// (compressed) secp256k1 verifying key.
//
// - `sig` is the 65-byte ECDSA signature (`r || s || recovery_id`).
// - `verifying_key` is the 33-byte compressed public key of the signer.
// - `prehash` is the 32-byte digest the signer hashed before signing, for
//   example, the output of SHA-256 or Keccak-256 over the original message.
//
// The caller is responsible for computing `prehash` with the same hash
// function the signer used; this verifier does no hashing of its own.
final fn verify_ecdsa_digest(sig: [u8; 65], verifying_key: [u8; 33], prehash: [u8; 32]) -> bool {
    return _ecdsa_verify_digest(sig, verifying_key, prehash);
}

// Verifies an ECDSA signature against a 32-byte prehash and a 20-byte
// Ethereum address. Use this variant when the signer is identified by their
// Ethereum address rather than by their raw secp256k1 public key, for
// example, when verifying signatures produced by MetaMask or any EVM
// wallet.
//
// - `sig` is the 65-byte ECDSA signature.
// - `eth_address` is the signer's 20-byte Ethereum address.
// - `prehash` is the 32-byte digest the signer hashed before signing.
final fn verify_ecdsa_digest_eth(sig: [u8; 65], eth_address: [u8; 20], prehash: [u8; 32]) -> bool {
    return _ecdsa_verify_digest_eth(sig, eth_address, prehash);
}
