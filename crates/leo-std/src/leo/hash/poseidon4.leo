// Poseidon-4 hashing, a SNARK-friendly sponge.
//
// Poseidon is designed for arithmetic circuits: every step is a field
// operation, so the hash costs an order of magnitude fewer constraints to
// prove than Keccak or SHA-3. Use Poseidon whenever the hash is computed
// and verified inside the same proof and EVM interop is not required.
//
// The numeric suffix is the sponge rate, the number of field elements
// absorbed per permutation. Higher rates absorb more data per step (cheaper
// per byte on long inputs) but provide proportionally less security
// margin; `poseidon2` is the safest default.
//
// # Variants
//
// Each "to_<T>" function returns the digest interpreted as a value of type
// `T`: `field`, an integer (truncated to its bit width), a curve
// point, or an Aleo address.
//
// `_raw` variants hash the input's bit pattern directly; the non-raw
// variants prepend a type tag so two values of different but bit-equal
// types do not collide.
//
// The input may be any value other than a mapping, tuple, or unit.

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `address`.
export fn hash_bool_to_address(x: bool) -> address {
    return _poseidon4_hash_to_address(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `field`.
export fn hash_bool_to_field(x: bool) -> field {
    return _poseidon4_hash_to_field(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `group`.
export fn hash_bool_to_group(x: bool) -> group {
    return _poseidon4_hash_to_group(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `scalar`.
export fn hash_bool_to_scalar(x: bool) -> scalar {
    return _poseidon4_hash_to_scalar(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `u8`.
export fn hash_bool_to_u8(x: bool) -> u8 {
    return _poseidon4_hash_to_u8(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `u16`.
export fn hash_bool_to_u16(x: bool) -> u16 {
    return _poseidon4_hash_to_u16(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `u32`.
export fn hash_bool_to_u32(x: bool) -> u32 {
    return _poseidon4_hash_to_u32(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `u64`.
export fn hash_bool_to_u64(x: bool) -> u64 {
    return _poseidon4_hash_to_u64(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `u128`.
export fn hash_bool_to_u128(x: bool) -> u128 {
    return _poseidon4_hash_to_u128(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `i8`.
export fn hash_bool_to_i8(x: bool) -> i8 {
    return _poseidon4_hash_to_i8(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `i16`.
export fn hash_bool_to_i16(x: bool) -> i16 {
    return _poseidon4_hash_to_i16(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `i32`.
export fn hash_bool_to_i32(x: bool) -> i32 {
    return _poseidon4_hash_to_i32(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `i64`.
export fn hash_bool_to_i64(x: bool) -> i64 {
    return _poseidon4_hash_to_i64(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `i128`.
export fn hash_bool_to_i128(x: bool) -> i128 {
    return _poseidon4_hash_to_i128(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `address`.
export fn hash_u8_to_address(x: u8) -> address {
    return _poseidon4_hash_to_address(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `field`.
export fn hash_u8_to_field(x: u8) -> field {
    return _poseidon4_hash_to_field(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `group`.
export fn hash_u8_to_group(x: u8) -> group {
    return _poseidon4_hash_to_group(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `scalar`.
export fn hash_u8_to_scalar(x: u8) -> scalar {
    return _poseidon4_hash_to_scalar(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `u8`.
export fn hash_u8_to_u8(x: u8) -> u8 {
    return _poseidon4_hash_to_u8(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `u16`.
export fn hash_u8_to_u16(x: u8) -> u16 {
    return _poseidon4_hash_to_u16(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `u32`.
export fn hash_u8_to_u32(x: u8) -> u32 {
    return _poseidon4_hash_to_u32(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `u64`.
export fn hash_u8_to_u64(x: u8) -> u64 {
    return _poseidon4_hash_to_u64(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `u128`.
export fn hash_u8_to_u128(x: u8) -> u128 {
    return _poseidon4_hash_to_u128(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `i8`.
export fn hash_u8_to_i8(x: u8) -> i8 {
    return _poseidon4_hash_to_i8(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `i16`.
export fn hash_u8_to_i16(x: u8) -> i16 {
    return _poseidon4_hash_to_i16(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `i32`.
export fn hash_u8_to_i32(x: u8) -> i32 {
    return _poseidon4_hash_to_i32(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `i64`.
export fn hash_u8_to_i64(x: u8) -> i64 {
    return _poseidon4_hash_to_i64(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `i128`.
export fn hash_u8_to_i128(x: u8) -> i128 {
    return _poseidon4_hash_to_i128(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `address`.
export fn hash_u16_to_address(x: u16) -> address {
    return _poseidon4_hash_to_address(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `field`.
export fn hash_u16_to_field(x: u16) -> field {
    return _poseidon4_hash_to_field(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `group`.
export fn hash_u16_to_group(x: u16) -> group {
    return _poseidon4_hash_to_group(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `scalar`.
export fn hash_u16_to_scalar(x: u16) -> scalar {
    return _poseidon4_hash_to_scalar(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `u8`.
export fn hash_u16_to_u8(x: u16) -> u8 {
    return _poseidon4_hash_to_u8(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `u16`.
export fn hash_u16_to_u16(x: u16) -> u16 {
    return _poseidon4_hash_to_u16(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `u32`.
export fn hash_u16_to_u32(x: u16) -> u32 {
    return _poseidon4_hash_to_u32(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `u64`.
export fn hash_u16_to_u64(x: u16) -> u64 {
    return _poseidon4_hash_to_u64(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `u128`.
export fn hash_u16_to_u128(x: u16) -> u128 {
    return _poseidon4_hash_to_u128(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `i8`.
export fn hash_u16_to_i8(x: u16) -> i8 {
    return _poseidon4_hash_to_i8(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `i16`.
export fn hash_u16_to_i16(x: u16) -> i16 {
    return _poseidon4_hash_to_i16(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `i32`.
export fn hash_u16_to_i32(x: u16) -> i32 {
    return _poseidon4_hash_to_i32(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `i64`.
export fn hash_u16_to_i64(x: u16) -> i64 {
    return _poseidon4_hash_to_i64(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `i128`.
export fn hash_u16_to_i128(x: u16) -> i128 {
    return _poseidon4_hash_to_i128(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `address`.
export fn hash_u32_to_address(x: u32) -> address {
    return _poseidon4_hash_to_address(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `field`.
export fn hash_u32_to_field(x: u32) -> field {
    return _poseidon4_hash_to_field(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `group`.
export fn hash_u32_to_group(x: u32) -> group {
    return _poseidon4_hash_to_group(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `scalar`.
export fn hash_u32_to_scalar(x: u32) -> scalar {
    return _poseidon4_hash_to_scalar(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `u8`.
export fn hash_u32_to_u8(x: u32) -> u8 {
    return _poseidon4_hash_to_u8(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `u16`.
export fn hash_u32_to_u16(x: u32) -> u16 {
    return _poseidon4_hash_to_u16(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `u32`.
export fn hash_u32_to_u32(x: u32) -> u32 {
    return _poseidon4_hash_to_u32(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `u64`.
export fn hash_u32_to_u64(x: u32) -> u64 {
    return _poseidon4_hash_to_u64(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `u128`.
export fn hash_u32_to_u128(x: u32) -> u128 {
    return _poseidon4_hash_to_u128(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `i8`.
export fn hash_u32_to_i8(x: u32) -> i8 {
    return _poseidon4_hash_to_i8(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `i16`.
export fn hash_u32_to_i16(x: u32) -> i16 {
    return _poseidon4_hash_to_i16(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `i32`.
export fn hash_u32_to_i32(x: u32) -> i32 {
    return _poseidon4_hash_to_i32(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `i64`.
export fn hash_u32_to_i64(x: u32) -> i64 {
    return _poseidon4_hash_to_i64(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `i128`.
export fn hash_u32_to_i128(x: u32) -> i128 {
    return _poseidon4_hash_to_i128(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `address`.
export fn hash_u64_to_address(x: u64) -> address {
    return _poseidon4_hash_to_address(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `field`.
export fn hash_u64_to_field(x: u64) -> field {
    return _poseidon4_hash_to_field(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `group`.
export fn hash_u64_to_group(x: u64) -> group {
    return _poseidon4_hash_to_group(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `scalar`.
export fn hash_u64_to_scalar(x: u64) -> scalar {
    return _poseidon4_hash_to_scalar(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `u8`.
export fn hash_u64_to_u8(x: u64) -> u8 {
    return _poseidon4_hash_to_u8(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `u16`.
export fn hash_u64_to_u16(x: u64) -> u16 {
    return _poseidon4_hash_to_u16(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `u32`.
export fn hash_u64_to_u32(x: u64) -> u32 {
    return _poseidon4_hash_to_u32(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `u64`.
export fn hash_u64_to_u64(x: u64) -> u64 {
    return _poseidon4_hash_to_u64(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `u128`.
export fn hash_u64_to_u128(x: u64) -> u128 {
    return _poseidon4_hash_to_u128(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `i8`.
export fn hash_u64_to_i8(x: u64) -> i8 {
    return _poseidon4_hash_to_i8(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `i16`.
export fn hash_u64_to_i16(x: u64) -> i16 {
    return _poseidon4_hash_to_i16(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `i32`.
export fn hash_u64_to_i32(x: u64) -> i32 {
    return _poseidon4_hash_to_i32(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `i64`.
export fn hash_u64_to_i64(x: u64) -> i64 {
    return _poseidon4_hash_to_i64(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `i128`.
export fn hash_u64_to_i128(x: u64) -> i128 {
    return _poseidon4_hash_to_i128(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `address`.
export fn hash_u128_to_address(x: u128) -> address {
    return _poseidon4_hash_to_address(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `field`.
export fn hash_u128_to_field(x: u128) -> field {
    return _poseidon4_hash_to_field(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `group`.
export fn hash_u128_to_group(x: u128) -> group {
    return _poseidon4_hash_to_group(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `scalar`.
export fn hash_u128_to_scalar(x: u128) -> scalar {
    return _poseidon4_hash_to_scalar(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `u8`.
export fn hash_u128_to_u8(x: u128) -> u8 {
    return _poseidon4_hash_to_u8(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `u16`.
export fn hash_u128_to_u16(x: u128) -> u16 {
    return _poseidon4_hash_to_u16(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `u32`.
export fn hash_u128_to_u32(x: u128) -> u32 {
    return _poseidon4_hash_to_u32(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `u64`.
export fn hash_u128_to_u64(x: u128) -> u64 {
    return _poseidon4_hash_to_u64(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `u128`.
export fn hash_u128_to_u128(x: u128) -> u128 {
    return _poseidon4_hash_to_u128(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `i8`.
export fn hash_u128_to_i8(x: u128) -> i8 {
    return _poseidon4_hash_to_i8(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `i16`.
export fn hash_u128_to_i16(x: u128) -> i16 {
    return _poseidon4_hash_to_i16(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `i32`.
export fn hash_u128_to_i32(x: u128) -> i32 {
    return _poseidon4_hash_to_i32(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `i64`.
export fn hash_u128_to_i64(x: u128) -> i64 {
    return _poseidon4_hash_to_i64(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `i128`.
export fn hash_u128_to_i128(x: u128) -> i128 {
    return _poseidon4_hash_to_i128(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `address`.
export fn hash_i8_to_address(x: i8) -> address {
    return _poseidon4_hash_to_address(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `field`.
export fn hash_i8_to_field(x: i8) -> field {
    return _poseidon4_hash_to_field(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `group`.
export fn hash_i8_to_group(x: i8) -> group {
    return _poseidon4_hash_to_group(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `scalar`.
export fn hash_i8_to_scalar(x: i8) -> scalar {
    return _poseidon4_hash_to_scalar(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `u8`.
export fn hash_i8_to_u8(x: i8) -> u8 {
    return _poseidon4_hash_to_u8(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `u16`.
export fn hash_i8_to_u16(x: i8) -> u16 {
    return _poseidon4_hash_to_u16(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `u32`.
export fn hash_i8_to_u32(x: i8) -> u32 {
    return _poseidon4_hash_to_u32(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `u64`.
export fn hash_i8_to_u64(x: i8) -> u64 {
    return _poseidon4_hash_to_u64(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `u128`.
export fn hash_i8_to_u128(x: i8) -> u128 {
    return _poseidon4_hash_to_u128(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `i8`.
export fn hash_i8_to_i8(x: i8) -> i8 {
    return _poseidon4_hash_to_i8(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `i16`.
export fn hash_i8_to_i16(x: i8) -> i16 {
    return _poseidon4_hash_to_i16(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `i32`.
export fn hash_i8_to_i32(x: i8) -> i32 {
    return _poseidon4_hash_to_i32(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `i64`.
export fn hash_i8_to_i64(x: i8) -> i64 {
    return _poseidon4_hash_to_i64(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `i128`.
export fn hash_i8_to_i128(x: i8) -> i128 {
    return _poseidon4_hash_to_i128(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `address`.
export fn hash_i16_to_address(x: i16) -> address {
    return _poseidon4_hash_to_address(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `field`.
export fn hash_i16_to_field(x: i16) -> field {
    return _poseidon4_hash_to_field(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `group`.
export fn hash_i16_to_group(x: i16) -> group {
    return _poseidon4_hash_to_group(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `scalar`.
export fn hash_i16_to_scalar(x: i16) -> scalar {
    return _poseidon4_hash_to_scalar(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `u8`.
export fn hash_i16_to_u8(x: i16) -> u8 {
    return _poseidon4_hash_to_u8(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `u16`.
export fn hash_i16_to_u16(x: i16) -> u16 {
    return _poseidon4_hash_to_u16(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `u32`.
export fn hash_i16_to_u32(x: i16) -> u32 {
    return _poseidon4_hash_to_u32(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `u64`.
export fn hash_i16_to_u64(x: i16) -> u64 {
    return _poseidon4_hash_to_u64(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `u128`.
export fn hash_i16_to_u128(x: i16) -> u128 {
    return _poseidon4_hash_to_u128(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `i8`.
export fn hash_i16_to_i8(x: i16) -> i8 {
    return _poseidon4_hash_to_i8(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `i16`.
export fn hash_i16_to_i16(x: i16) -> i16 {
    return _poseidon4_hash_to_i16(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `i32`.
export fn hash_i16_to_i32(x: i16) -> i32 {
    return _poseidon4_hash_to_i32(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `i64`.
export fn hash_i16_to_i64(x: i16) -> i64 {
    return _poseidon4_hash_to_i64(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `i128`.
export fn hash_i16_to_i128(x: i16) -> i128 {
    return _poseidon4_hash_to_i128(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `address`.
export fn hash_i32_to_address(x: i32) -> address {
    return _poseidon4_hash_to_address(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `field`.
export fn hash_i32_to_field(x: i32) -> field {
    return _poseidon4_hash_to_field(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `group`.
export fn hash_i32_to_group(x: i32) -> group {
    return _poseidon4_hash_to_group(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `scalar`.
export fn hash_i32_to_scalar(x: i32) -> scalar {
    return _poseidon4_hash_to_scalar(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `u8`.
export fn hash_i32_to_u8(x: i32) -> u8 {
    return _poseidon4_hash_to_u8(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `u16`.
export fn hash_i32_to_u16(x: i32) -> u16 {
    return _poseidon4_hash_to_u16(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `u32`.
export fn hash_i32_to_u32(x: i32) -> u32 {
    return _poseidon4_hash_to_u32(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `u64`.
export fn hash_i32_to_u64(x: i32) -> u64 {
    return _poseidon4_hash_to_u64(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `u128`.
export fn hash_i32_to_u128(x: i32) -> u128 {
    return _poseidon4_hash_to_u128(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `i8`.
export fn hash_i32_to_i8(x: i32) -> i8 {
    return _poseidon4_hash_to_i8(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `i16`.
export fn hash_i32_to_i16(x: i32) -> i16 {
    return _poseidon4_hash_to_i16(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `i32`.
export fn hash_i32_to_i32(x: i32) -> i32 {
    return _poseidon4_hash_to_i32(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `i64`.
export fn hash_i32_to_i64(x: i32) -> i64 {
    return _poseidon4_hash_to_i64(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `i128`.
export fn hash_i32_to_i128(x: i32) -> i128 {
    return _poseidon4_hash_to_i128(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `address`.
export fn hash_i64_to_address(x: i64) -> address {
    return _poseidon4_hash_to_address(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `field`.
export fn hash_i64_to_field(x: i64) -> field {
    return _poseidon4_hash_to_field(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `group`.
export fn hash_i64_to_group(x: i64) -> group {
    return _poseidon4_hash_to_group(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `scalar`.
export fn hash_i64_to_scalar(x: i64) -> scalar {
    return _poseidon4_hash_to_scalar(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `u8`.
export fn hash_i64_to_u8(x: i64) -> u8 {
    return _poseidon4_hash_to_u8(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `u16`.
export fn hash_i64_to_u16(x: i64) -> u16 {
    return _poseidon4_hash_to_u16(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `u32`.
export fn hash_i64_to_u32(x: i64) -> u32 {
    return _poseidon4_hash_to_u32(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `u64`.
export fn hash_i64_to_u64(x: i64) -> u64 {
    return _poseidon4_hash_to_u64(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `u128`.
export fn hash_i64_to_u128(x: i64) -> u128 {
    return _poseidon4_hash_to_u128(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `i8`.
export fn hash_i64_to_i8(x: i64) -> i8 {
    return _poseidon4_hash_to_i8(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `i16`.
export fn hash_i64_to_i16(x: i64) -> i16 {
    return _poseidon4_hash_to_i16(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `i32`.
export fn hash_i64_to_i32(x: i64) -> i32 {
    return _poseidon4_hash_to_i32(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `i64`.
export fn hash_i64_to_i64(x: i64) -> i64 {
    return _poseidon4_hash_to_i64(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `i128`.
export fn hash_i64_to_i128(x: i64) -> i128 {
    return _poseidon4_hash_to_i128(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `address`.
export fn hash_i128_to_address(x: i128) -> address {
    return _poseidon4_hash_to_address(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `field`.
export fn hash_i128_to_field(x: i128) -> field {
    return _poseidon4_hash_to_field(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `group`.
export fn hash_i128_to_group(x: i128) -> group {
    return _poseidon4_hash_to_group(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `scalar`.
export fn hash_i128_to_scalar(x: i128) -> scalar {
    return _poseidon4_hash_to_scalar(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `u8`.
export fn hash_i128_to_u8(x: i128) -> u8 {
    return _poseidon4_hash_to_u8(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `u16`.
export fn hash_i128_to_u16(x: i128) -> u16 {
    return _poseidon4_hash_to_u16(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `u32`.
export fn hash_i128_to_u32(x: i128) -> u32 {
    return _poseidon4_hash_to_u32(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `u64`.
export fn hash_i128_to_u64(x: i128) -> u64 {
    return _poseidon4_hash_to_u64(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `u128`.
export fn hash_i128_to_u128(x: i128) -> u128 {
    return _poseidon4_hash_to_u128(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `i8`.
export fn hash_i128_to_i8(x: i128) -> i8 {
    return _poseidon4_hash_to_i8(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `i16`.
export fn hash_i128_to_i16(x: i128) -> i16 {
    return _poseidon4_hash_to_i16(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `i32`.
export fn hash_i128_to_i32(x: i128) -> i32 {
    return _poseidon4_hash_to_i32(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `i64`.
export fn hash_i128_to_i64(x: i128) -> i64 {
    return _poseidon4_hash_to_i64(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `i128`.
export fn hash_i128_to_i128(x: i128) -> i128 {
    return _poseidon4_hash_to_i128(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `address`.
export fn hash_field_to_address(x: field) -> address {
    return _poseidon4_hash_to_address(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `field`.
export fn hash_field_to_field(x: field) -> field {
    return _poseidon4_hash_to_field(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `group`.
export fn hash_field_to_group(x: field) -> group {
    return _poseidon4_hash_to_group(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `scalar`.
export fn hash_field_to_scalar(x: field) -> scalar {
    return _poseidon4_hash_to_scalar(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `u8`.
export fn hash_field_to_u8(x: field) -> u8 {
    return _poseidon4_hash_to_u8(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `u16`.
export fn hash_field_to_u16(x: field) -> u16 {
    return _poseidon4_hash_to_u16(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `u32`.
export fn hash_field_to_u32(x: field) -> u32 {
    return _poseidon4_hash_to_u32(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `u64`.
export fn hash_field_to_u64(x: field) -> u64 {
    return _poseidon4_hash_to_u64(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `u128`.
export fn hash_field_to_u128(x: field) -> u128 {
    return _poseidon4_hash_to_u128(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `i8`.
export fn hash_field_to_i8(x: field) -> i8 {
    return _poseidon4_hash_to_i8(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `i16`.
export fn hash_field_to_i16(x: field) -> i16 {
    return _poseidon4_hash_to_i16(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `i32`.
export fn hash_field_to_i32(x: field) -> i32 {
    return _poseidon4_hash_to_i32(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `i64`.
export fn hash_field_to_i64(x: field) -> i64 {
    return _poseidon4_hash_to_i64(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `i128`.
export fn hash_field_to_i128(x: field) -> i128 {
    return _poseidon4_hash_to_i128(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `address`.
export fn hash_group_to_address(x: group) -> address {
    return _poseidon4_hash_to_address(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `field`.
export fn hash_group_to_field(x: group) -> field {
    return _poseidon4_hash_to_field(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `group`.
export fn hash_group_to_group(x: group) -> group {
    return _poseidon4_hash_to_group(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `scalar`.
export fn hash_group_to_scalar(x: group) -> scalar {
    return _poseidon4_hash_to_scalar(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `u8`.
export fn hash_group_to_u8(x: group) -> u8 {
    return _poseidon4_hash_to_u8(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `u16`.
export fn hash_group_to_u16(x: group) -> u16 {
    return _poseidon4_hash_to_u16(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `u32`.
export fn hash_group_to_u32(x: group) -> u32 {
    return _poseidon4_hash_to_u32(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `u64`.
export fn hash_group_to_u64(x: group) -> u64 {
    return _poseidon4_hash_to_u64(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `u128`.
export fn hash_group_to_u128(x: group) -> u128 {
    return _poseidon4_hash_to_u128(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `i8`.
export fn hash_group_to_i8(x: group) -> i8 {
    return _poseidon4_hash_to_i8(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `i16`.
export fn hash_group_to_i16(x: group) -> i16 {
    return _poseidon4_hash_to_i16(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `i32`.
export fn hash_group_to_i32(x: group) -> i32 {
    return _poseidon4_hash_to_i32(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `i64`.
export fn hash_group_to_i64(x: group) -> i64 {
    return _poseidon4_hash_to_i64(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `i128`.
export fn hash_group_to_i128(x: group) -> i128 {
    return _poseidon4_hash_to_i128(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `address`.
export fn hash_scalar_to_address(x: scalar) -> address {
    return _poseidon4_hash_to_address(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `field`.
export fn hash_scalar_to_field(x: scalar) -> field {
    return _poseidon4_hash_to_field(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `group`.
export fn hash_scalar_to_group(x: scalar) -> group {
    return _poseidon4_hash_to_group(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `scalar`.
export fn hash_scalar_to_scalar(x: scalar) -> scalar {
    return _poseidon4_hash_to_scalar(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `u8`.
export fn hash_scalar_to_u8(x: scalar) -> u8 {
    return _poseidon4_hash_to_u8(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `u16`.
export fn hash_scalar_to_u16(x: scalar) -> u16 {
    return _poseidon4_hash_to_u16(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `u32`.
export fn hash_scalar_to_u32(x: scalar) -> u32 {
    return _poseidon4_hash_to_u32(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `u64`.
export fn hash_scalar_to_u64(x: scalar) -> u64 {
    return _poseidon4_hash_to_u64(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `u128`.
export fn hash_scalar_to_u128(x: scalar) -> u128 {
    return _poseidon4_hash_to_u128(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `i8`.
export fn hash_scalar_to_i8(x: scalar) -> i8 {
    return _poseidon4_hash_to_i8(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `i16`.
export fn hash_scalar_to_i16(x: scalar) -> i16 {
    return _poseidon4_hash_to_i16(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `i32`.
export fn hash_scalar_to_i32(x: scalar) -> i32 {
    return _poseidon4_hash_to_i32(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `i64`.
export fn hash_scalar_to_i64(x: scalar) -> i64 {
    return _poseidon4_hash_to_i64(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `i128`.
export fn hash_scalar_to_i128(x: scalar) -> i128 {
    return _poseidon4_hash_to_i128(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `address`.
export fn hash_address_to_address(x: address) -> address {
    return _poseidon4_hash_to_address(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `field`.
export fn hash_address_to_field(x: address) -> field {
    return _poseidon4_hash_to_field(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `group`.
export fn hash_address_to_group(x: address) -> group {
    return _poseidon4_hash_to_group(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `scalar`.
export fn hash_address_to_scalar(x: address) -> scalar {
    return _poseidon4_hash_to_scalar(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `u8`.
export fn hash_address_to_u8(x: address) -> u8 {
    return _poseidon4_hash_to_u8(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `u16`.
export fn hash_address_to_u16(x: address) -> u16 {
    return _poseidon4_hash_to_u16(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `u32`.
export fn hash_address_to_u32(x: address) -> u32 {
    return _poseidon4_hash_to_u32(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `u64`.
export fn hash_address_to_u64(x: address) -> u64 {
    return _poseidon4_hash_to_u64(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `u128`.
export fn hash_address_to_u128(x: address) -> u128 {
    return _poseidon4_hash_to_u128(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `i8`.
export fn hash_address_to_i8(x: address) -> i8 {
    return _poseidon4_hash_to_i8(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `i16`.
export fn hash_address_to_i16(x: address) -> i16 {
    return _poseidon4_hash_to_i16(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `i32`.
export fn hash_address_to_i32(x: address) -> i32 {
    return _poseidon4_hash_to_i32(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `i64`.
export fn hash_address_to_i64(x: address) -> i64 {
    return _poseidon4_hash_to_i64(x);
}

// Hashes `x` with Poseidon-4 (tagged input encoding) and returns the digest as `i128`.
export fn hash_address_to_i128(x: address) -> i128 {
    return _poseidon4_hash_to_i128(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `address`.
export fn hash_bool_to_address_raw(x: bool) -> address {
    return _poseidon4_hash_to_address_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `field`.
export fn hash_bool_to_field_raw(x: bool) -> field {
    return _poseidon4_hash_to_field_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `group`.
export fn hash_bool_to_group_raw(x: bool) -> group {
    return _poseidon4_hash_to_group_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `scalar`.
export fn hash_bool_to_scalar_raw(x: bool) -> scalar {
    return _poseidon4_hash_to_scalar_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `u8`.
export fn hash_bool_to_u8_raw(x: bool) -> u8 {
    return _poseidon4_hash_to_u8_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `u16`.
export fn hash_bool_to_u16_raw(x: bool) -> u16 {
    return _poseidon4_hash_to_u16_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `u32`.
export fn hash_bool_to_u32_raw(x: bool) -> u32 {
    return _poseidon4_hash_to_u32_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `u64`.
export fn hash_bool_to_u64_raw(x: bool) -> u64 {
    return _poseidon4_hash_to_u64_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `u128`.
export fn hash_bool_to_u128_raw(x: bool) -> u128 {
    return _poseidon4_hash_to_u128_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `i8`.
export fn hash_bool_to_i8_raw(x: bool) -> i8 {
    return _poseidon4_hash_to_i8_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `i16`.
export fn hash_bool_to_i16_raw(x: bool) -> i16 {
    return _poseidon4_hash_to_i16_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `i32`.
export fn hash_bool_to_i32_raw(x: bool) -> i32 {
    return _poseidon4_hash_to_i32_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `i64`.
export fn hash_bool_to_i64_raw(x: bool) -> i64 {
    return _poseidon4_hash_to_i64_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `i128`.
export fn hash_bool_to_i128_raw(x: bool) -> i128 {
    return _poseidon4_hash_to_i128_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `address`.
export fn hash_u8_to_address_raw(x: u8) -> address {
    return _poseidon4_hash_to_address_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `field`.
export fn hash_u8_to_field_raw(x: u8) -> field {
    return _poseidon4_hash_to_field_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `group`.
export fn hash_u8_to_group_raw(x: u8) -> group {
    return _poseidon4_hash_to_group_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `scalar`.
export fn hash_u8_to_scalar_raw(x: u8) -> scalar {
    return _poseidon4_hash_to_scalar_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `u8`.
export fn hash_u8_to_u8_raw(x: u8) -> u8 {
    return _poseidon4_hash_to_u8_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `u16`.
export fn hash_u8_to_u16_raw(x: u8) -> u16 {
    return _poseidon4_hash_to_u16_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `u32`.
export fn hash_u8_to_u32_raw(x: u8) -> u32 {
    return _poseidon4_hash_to_u32_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `u64`.
export fn hash_u8_to_u64_raw(x: u8) -> u64 {
    return _poseidon4_hash_to_u64_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `u128`.
export fn hash_u8_to_u128_raw(x: u8) -> u128 {
    return _poseidon4_hash_to_u128_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `i8`.
export fn hash_u8_to_i8_raw(x: u8) -> i8 {
    return _poseidon4_hash_to_i8_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `i16`.
export fn hash_u8_to_i16_raw(x: u8) -> i16 {
    return _poseidon4_hash_to_i16_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `i32`.
export fn hash_u8_to_i32_raw(x: u8) -> i32 {
    return _poseidon4_hash_to_i32_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `i64`.
export fn hash_u8_to_i64_raw(x: u8) -> i64 {
    return _poseidon4_hash_to_i64_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `i128`.
export fn hash_u8_to_i128_raw(x: u8) -> i128 {
    return _poseidon4_hash_to_i128_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `address`.
export fn hash_u16_to_address_raw(x: u16) -> address {
    return _poseidon4_hash_to_address_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `field`.
export fn hash_u16_to_field_raw(x: u16) -> field {
    return _poseidon4_hash_to_field_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `group`.
export fn hash_u16_to_group_raw(x: u16) -> group {
    return _poseidon4_hash_to_group_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `scalar`.
export fn hash_u16_to_scalar_raw(x: u16) -> scalar {
    return _poseidon4_hash_to_scalar_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `u8`.
export fn hash_u16_to_u8_raw(x: u16) -> u8 {
    return _poseidon4_hash_to_u8_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `u16`.
export fn hash_u16_to_u16_raw(x: u16) -> u16 {
    return _poseidon4_hash_to_u16_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `u32`.
export fn hash_u16_to_u32_raw(x: u16) -> u32 {
    return _poseidon4_hash_to_u32_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `u64`.
export fn hash_u16_to_u64_raw(x: u16) -> u64 {
    return _poseidon4_hash_to_u64_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `u128`.
export fn hash_u16_to_u128_raw(x: u16) -> u128 {
    return _poseidon4_hash_to_u128_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `i8`.
export fn hash_u16_to_i8_raw(x: u16) -> i8 {
    return _poseidon4_hash_to_i8_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `i16`.
export fn hash_u16_to_i16_raw(x: u16) -> i16 {
    return _poseidon4_hash_to_i16_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `i32`.
export fn hash_u16_to_i32_raw(x: u16) -> i32 {
    return _poseidon4_hash_to_i32_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `i64`.
export fn hash_u16_to_i64_raw(x: u16) -> i64 {
    return _poseidon4_hash_to_i64_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `i128`.
export fn hash_u16_to_i128_raw(x: u16) -> i128 {
    return _poseidon4_hash_to_i128_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `address`.
export fn hash_u32_to_address_raw(x: u32) -> address {
    return _poseidon4_hash_to_address_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `field`.
export fn hash_u32_to_field_raw(x: u32) -> field {
    return _poseidon4_hash_to_field_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `group`.
export fn hash_u32_to_group_raw(x: u32) -> group {
    return _poseidon4_hash_to_group_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `scalar`.
export fn hash_u32_to_scalar_raw(x: u32) -> scalar {
    return _poseidon4_hash_to_scalar_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `u8`.
export fn hash_u32_to_u8_raw(x: u32) -> u8 {
    return _poseidon4_hash_to_u8_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `u16`.
export fn hash_u32_to_u16_raw(x: u32) -> u16 {
    return _poseidon4_hash_to_u16_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `u32`.
export fn hash_u32_to_u32_raw(x: u32) -> u32 {
    return _poseidon4_hash_to_u32_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `u64`.
export fn hash_u32_to_u64_raw(x: u32) -> u64 {
    return _poseidon4_hash_to_u64_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `u128`.
export fn hash_u32_to_u128_raw(x: u32) -> u128 {
    return _poseidon4_hash_to_u128_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `i8`.
export fn hash_u32_to_i8_raw(x: u32) -> i8 {
    return _poseidon4_hash_to_i8_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `i16`.
export fn hash_u32_to_i16_raw(x: u32) -> i16 {
    return _poseidon4_hash_to_i16_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `i32`.
export fn hash_u32_to_i32_raw(x: u32) -> i32 {
    return _poseidon4_hash_to_i32_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `i64`.
export fn hash_u32_to_i64_raw(x: u32) -> i64 {
    return _poseidon4_hash_to_i64_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `i128`.
export fn hash_u32_to_i128_raw(x: u32) -> i128 {
    return _poseidon4_hash_to_i128_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `address`.
export fn hash_u64_to_address_raw(x: u64) -> address {
    return _poseidon4_hash_to_address_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `field`.
export fn hash_u64_to_field_raw(x: u64) -> field {
    return _poseidon4_hash_to_field_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `group`.
export fn hash_u64_to_group_raw(x: u64) -> group {
    return _poseidon4_hash_to_group_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `scalar`.
export fn hash_u64_to_scalar_raw(x: u64) -> scalar {
    return _poseidon4_hash_to_scalar_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `u8`.
export fn hash_u64_to_u8_raw(x: u64) -> u8 {
    return _poseidon4_hash_to_u8_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `u16`.
export fn hash_u64_to_u16_raw(x: u64) -> u16 {
    return _poseidon4_hash_to_u16_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `u32`.
export fn hash_u64_to_u32_raw(x: u64) -> u32 {
    return _poseidon4_hash_to_u32_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `u64`.
export fn hash_u64_to_u64_raw(x: u64) -> u64 {
    return _poseidon4_hash_to_u64_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `u128`.
export fn hash_u64_to_u128_raw(x: u64) -> u128 {
    return _poseidon4_hash_to_u128_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `i8`.
export fn hash_u64_to_i8_raw(x: u64) -> i8 {
    return _poseidon4_hash_to_i8_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `i16`.
export fn hash_u64_to_i16_raw(x: u64) -> i16 {
    return _poseidon4_hash_to_i16_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `i32`.
export fn hash_u64_to_i32_raw(x: u64) -> i32 {
    return _poseidon4_hash_to_i32_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `i64`.
export fn hash_u64_to_i64_raw(x: u64) -> i64 {
    return _poseidon4_hash_to_i64_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `i128`.
export fn hash_u64_to_i128_raw(x: u64) -> i128 {
    return _poseidon4_hash_to_i128_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `address`.
export fn hash_u128_to_address_raw(x: u128) -> address {
    return _poseidon4_hash_to_address_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `field`.
export fn hash_u128_to_field_raw(x: u128) -> field {
    return _poseidon4_hash_to_field_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `group`.
export fn hash_u128_to_group_raw(x: u128) -> group {
    return _poseidon4_hash_to_group_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `scalar`.
export fn hash_u128_to_scalar_raw(x: u128) -> scalar {
    return _poseidon4_hash_to_scalar_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `u8`.
export fn hash_u128_to_u8_raw(x: u128) -> u8 {
    return _poseidon4_hash_to_u8_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `u16`.
export fn hash_u128_to_u16_raw(x: u128) -> u16 {
    return _poseidon4_hash_to_u16_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `u32`.
export fn hash_u128_to_u32_raw(x: u128) -> u32 {
    return _poseidon4_hash_to_u32_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `u64`.
export fn hash_u128_to_u64_raw(x: u128) -> u64 {
    return _poseidon4_hash_to_u64_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `u128`.
export fn hash_u128_to_u128_raw(x: u128) -> u128 {
    return _poseidon4_hash_to_u128_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `i8`.
export fn hash_u128_to_i8_raw(x: u128) -> i8 {
    return _poseidon4_hash_to_i8_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `i16`.
export fn hash_u128_to_i16_raw(x: u128) -> i16 {
    return _poseidon4_hash_to_i16_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `i32`.
export fn hash_u128_to_i32_raw(x: u128) -> i32 {
    return _poseidon4_hash_to_i32_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `i64`.
export fn hash_u128_to_i64_raw(x: u128) -> i64 {
    return _poseidon4_hash_to_i64_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `i128`.
export fn hash_u128_to_i128_raw(x: u128) -> i128 {
    return _poseidon4_hash_to_i128_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `address`.
export fn hash_i8_to_address_raw(x: i8) -> address {
    return _poseidon4_hash_to_address_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `field`.
export fn hash_i8_to_field_raw(x: i8) -> field {
    return _poseidon4_hash_to_field_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `group`.
export fn hash_i8_to_group_raw(x: i8) -> group {
    return _poseidon4_hash_to_group_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `scalar`.
export fn hash_i8_to_scalar_raw(x: i8) -> scalar {
    return _poseidon4_hash_to_scalar_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `u8`.
export fn hash_i8_to_u8_raw(x: i8) -> u8 {
    return _poseidon4_hash_to_u8_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `u16`.
export fn hash_i8_to_u16_raw(x: i8) -> u16 {
    return _poseidon4_hash_to_u16_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `u32`.
export fn hash_i8_to_u32_raw(x: i8) -> u32 {
    return _poseidon4_hash_to_u32_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `u64`.
export fn hash_i8_to_u64_raw(x: i8) -> u64 {
    return _poseidon4_hash_to_u64_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `u128`.
export fn hash_i8_to_u128_raw(x: i8) -> u128 {
    return _poseidon4_hash_to_u128_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `i8`.
export fn hash_i8_to_i8_raw(x: i8) -> i8 {
    return _poseidon4_hash_to_i8_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `i16`.
export fn hash_i8_to_i16_raw(x: i8) -> i16 {
    return _poseidon4_hash_to_i16_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `i32`.
export fn hash_i8_to_i32_raw(x: i8) -> i32 {
    return _poseidon4_hash_to_i32_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `i64`.
export fn hash_i8_to_i64_raw(x: i8) -> i64 {
    return _poseidon4_hash_to_i64_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `i128`.
export fn hash_i8_to_i128_raw(x: i8) -> i128 {
    return _poseidon4_hash_to_i128_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `address`.
export fn hash_i16_to_address_raw(x: i16) -> address {
    return _poseidon4_hash_to_address_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `field`.
export fn hash_i16_to_field_raw(x: i16) -> field {
    return _poseidon4_hash_to_field_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `group`.
export fn hash_i16_to_group_raw(x: i16) -> group {
    return _poseidon4_hash_to_group_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `scalar`.
export fn hash_i16_to_scalar_raw(x: i16) -> scalar {
    return _poseidon4_hash_to_scalar_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `u8`.
export fn hash_i16_to_u8_raw(x: i16) -> u8 {
    return _poseidon4_hash_to_u8_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `u16`.
export fn hash_i16_to_u16_raw(x: i16) -> u16 {
    return _poseidon4_hash_to_u16_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `u32`.
export fn hash_i16_to_u32_raw(x: i16) -> u32 {
    return _poseidon4_hash_to_u32_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `u64`.
export fn hash_i16_to_u64_raw(x: i16) -> u64 {
    return _poseidon4_hash_to_u64_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `u128`.
export fn hash_i16_to_u128_raw(x: i16) -> u128 {
    return _poseidon4_hash_to_u128_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `i8`.
export fn hash_i16_to_i8_raw(x: i16) -> i8 {
    return _poseidon4_hash_to_i8_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `i16`.
export fn hash_i16_to_i16_raw(x: i16) -> i16 {
    return _poseidon4_hash_to_i16_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `i32`.
export fn hash_i16_to_i32_raw(x: i16) -> i32 {
    return _poseidon4_hash_to_i32_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `i64`.
export fn hash_i16_to_i64_raw(x: i16) -> i64 {
    return _poseidon4_hash_to_i64_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `i128`.
export fn hash_i16_to_i128_raw(x: i16) -> i128 {
    return _poseidon4_hash_to_i128_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `address`.
export fn hash_i32_to_address_raw(x: i32) -> address {
    return _poseidon4_hash_to_address_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `field`.
export fn hash_i32_to_field_raw(x: i32) -> field {
    return _poseidon4_hash_to_field_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `group`.
export fn hash_i32_to_group_raw(x: i32) -> group {
    return _poseidon4_hash_to_group_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `scalar`.
export fn hash_i32_to_scalar_raw(x: i32) -> scalar {
    return _poseidon4_hash_to_scalar_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `u8`.
export fn hash_i32_to_u8_raw(x: i32) -> u8 {
    return _poseidon4_hash_to_u8_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `u16`.
export fn hash_i32_to_u16_raw(x: i32) -> u16 {
    return _poseidon4_hash_to_u16_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `u32`.
export fn hash_i32_to_u32_raw(x: i32) -> u32 {
    return _poseidon4_hash_to_u32_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `u64`.
export fn hash_i32_to_u64_raw(x: i32) -> u64 {
    return _poseidon4_hash_to_u64_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `u128`.
export fn hash_i32_to_u128_raw(x: i32) -> u128 {
    return _poseidon4_hash_to_u128_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `i8`.
export fn hash_i32_to_i8_raw(x: i32) -> i8 {
    return _poseidon4_hash_to_i8_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `i16`.
export fn hash_i32_to_i16_raw(x: i32) -> i16 {
    return _poseidon4_hash_to_i16_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `i32`.
export fn hash_i32_to_i32_raw(x: i32) -> i32 {
    return _poseidon4_hash_to_i32_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `i64`.
export fn hash_i32_to_i64_raw(x: i32) -> i64 {
    return _poseidon4_hash_to_i64_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `i128`.
export fn hash_i32_to_i128_raw(x: i32) -> i128 {
    return _poseidon4_hash_to_i128_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `address`.
export fn hash_i64_to_address_raw(x: i64) -> address {
    return _poseidon4_hash_to_address_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `field`.
export fn hash_i64_to_field_raw(x: i64) -> field {
    return _poseidon4_hash_to_field_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `group`.
export fn hash_i64_to_group_raw(x: i64) -> group {
    return _poseidon4_hash_to_group_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `scalar`.
export fn hash_i64_to_scalar_raw(x: i64) -> scalar {
    return _poseidon4_hash_to_scalar_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `u8`.
export fn hash_i64_to_u8_raw(x: i64) -> u8 {
    return _poseidon4_hash_to_u8_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `u16`.
export fn hash_i64_to_u16_raw(x: i64) -> u16 {
    return _poseidon4_hash_to_u16_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `u32`.
export fn hash_i64_to_u32_raw(x: i64) -> u32 {
    return _poseidon4_hash_to_u32_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `u64`.
export fn hash_i64_to_u64_raw(x: i64) -> u64 {
    return _poseidon4_hash_to_u64_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `u128`.
export fn hash_i64_to_u128_raw(x: i64) -> u128 {
    return _poseidon4_hash_to_u128_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `i8`.
export fn hash_i64_to_i8_raw(x: i64) -> i8 {
    return _poseidon4_hash_to_i8_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `i16`.
export fn hash_i64_to_i16_raw(x: i64) -> i16 {
    return _poseidon4_hash_to_i16_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `i32`.
export fn hash_i64_to_i32_raw(x: i64) -> i32 {
    return _poseidon4_hash_to_i32_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `i64`.
export fn hash_i64_to_i64_raw(x: i64) -> i64 {
    return _poseidon4_hash_to_i64_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `i128`.
export fn hash_i64_to_i128_raw(x: i64) -> i128 {
    return _poseidon4_hash_to_i128_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `address`.
export fn hash_i128_to_address_raw(x: i128) -> address {
    return _poseidon4_hash_to_address_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `field`.
export fn hash_i128_to_field_raw(x: i128) -> field {
    return _poseidon4_hash_to_field_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `group`.
export fn hash_i128_to_group_raw(x: i128) -> group {
    return _poseidon4_hash_to_group_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `scalar`.
export fn hash_i128_to_scalar_raw(x: i128) -> scalar {
    return _poseidon4_hash_to_scalar_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `u8`.
export fn hash_i128_to_u8_raw(x: i128) -> u8 {
    return _poseidon4_hash_to_u8_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `u16`.
export fn hash_i128_to_u16_raw(x: i128) -> u16 {
    return _poseidon4_hash_to_u16_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `u32`.
export fn hash_i128_to_u32_raw(x: i128) -> u32 {
    return _poseidon4_hash_to_u32_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `u64`.
export fn hash_i128_to_u64_raw(x: i128) -> u64 {
    return _poseidon4_hash_to_u64_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `u128`.
export fn hash_i128_to_u128_raw(x: i128) -> u128 {
    return _poseidon4_hash_to_u128_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `i8`.
export fn hash_i128_to_i8_raw(x: i128) -> i8 {
    return _poseidon4_hash_to_i8_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `i16`.
export fn hash_i128_to_i16_raw(x: i128) -> i16 {
    return _poseidon4_hash_to_i16_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `i32`.
export fn hash_i128_to_i32_raw(x: i128) -> i32 {
    return _poseidon4_hash_to_i32_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `i64`.
export fn hash_i128_to_i64_raw(x: i128) -> i64 {
    return _poseidon4_hash_to_i64_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `i128`.
export fn hash_i128_to_i128_raw(x: i128) -> i128 {
    return _poseidon4_hash_to_i128_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `address`.
export fn hash_field_to_address_raw(x: field) -> address {
    return _poseidon4_hash_to_address_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `field`.
export fn hash_field_to_field_raw(x: field) -> field {
    return _poseidon4_hash_to_field_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `group`.
export fn hash_field_to_group_raw(x: field) -> group {
    return _poseidon4_hash_to_group_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `scalar`.
export fn hash_field_to_scalar_raw(x: field) -> scalar {
    return _poseidon4_hash_to_scalar_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `u8`.
export fn hash_field_to_u8_raw(x: field) -> u8 {
    return _poseidon4_hash_to_u8_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `u16`.
export fn hash_field_to_u16_raw(x: field) -> u16 {
    return _poseidon4_hash_to_u16_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `u32`.
export fn hash_field_to_u32_raw(x: field) -> u32 {
    return _poseidon4_hash_to_u32_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `u64`.
export fn hash_field_to_u64_raw(x: field) -> u64 {
    return _poseidon4_hash_to_u64_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `u128`.
export fn hash_field_to_u128_raw(x: field) -> u128 {
    return _poseidon4_hash_to_u128_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `i8`.
export fn hash_field_to_i8_raw(x: field) -> i8 {
    return _poseidon4_hash_to_i8_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `i16`.
export fn hash_field_to_i16_raw(x: field) -> i16 {
    return _poseidon4_hash_to_i16_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `i32`.
export fn hash_field_to_i32_raw(x: field) -> i32 {
    return _poseidon4_hash_to_i32_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `i64`.
export fn hash_field_to_i64_raw(x: field) -> i64 {
    return _poseidon4_hash_to_i64_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `i128`.
export fn hash_field_to_i128_raw(x: field) -> i128 {
    return _poseidon4_hash_to_i128_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `address`.
export fn hash_group_to_address_raw(x: group) -> address {
    return _poseidon4_hash_to_address_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `field`.
export fn hash_group_to_field_raw(x: group) -> field {
    return _poseidon4_hash_to_field_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `group`.
export fn hash_group_to_group_raw(x: group) -> group {
    return _poseidon4_hash_to_group_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `scalar`.
export fn hash_group_to_scalar_raw(x: group) -> scalar {
    return _poseidon4_hash_to_scalar_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `u8`.
export fn hash_group_to_u8_raw(x: group) -> u8 {
    return _poseidon4_hash_to_u8_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `u16`.
export fn hash_group_to_u16_raw(x: group) -> u16 {
    return _poseidon4_hash_to_u16_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `u32`.
export fn hash_group_to_u32_raw(x: group) -> u32 {
    return _poseidon4_hash_to_u32_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `u64`.
export fn hash_group_to_u64_raw(x: group) -> u64 {
    return _poseidon4_hash_to_u64_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `u128`.
export fn hash_group_to_u128_raw(x: group) -> u128 {
    return _poseidon4_hash_to_u128_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `i8`.
export fn hash_group_to_i8_raw(x: group) -> i8 {
    return _poseidon4_hash_to_i8_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `i16`.
export fn hash_group_to_i16_raw(x: group) -> i16 {
    return _poseidon4_hash_to_i16_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `i32`.
export fn hash_group_to_i32_raw(x: group) -> i32 {
    return _poseidon4_hash_to_i32_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `i64`.
export fn hash_group_to_i64_raw(x: group) -> i64 {
    return _poseidon4_hash_to_i64_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `i128`.
export fn hash_group_to_i128_raw(x: group) -> i128 {
    return _poseidon4_hash_to_i128_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `address`.
export fn hash_scalar_to_address_raw(x: scalar) -> address {
    return _poseidon4_hash_to_address_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `field`.
export fn hash_scalar_to_field_raw(x: scalar) -> field {
    return _poseidon4_hash_to_field_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `group`.
export fn hash_scalar_to_group_raw(x: scalar) -> group {
    return _poseidon4_hash_to_group_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `scalar`.
export fn hash_scalar_to_scalar_raw(x: scalar) -> scalar {
    return _poseidon4_hash_to_scalar_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `u8`.
export fn hash_scalar_to_u8_raw(x: scalar) -> u8 {
    return _poseidon4_hash_to_u8_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `u16`.
export fn hash_scalar_to_u16_raw(x: scalar) -> u16 {
    return _poseidon4_hash_to_u16_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `u32`.
export fn hash_scalar_to_u32_raw(x: scalar) -> u32 {
    return _poseidon4_hash_to_u32_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `u64`.
export fn hash_scalar_to_u64_raw(x: scalar) -> u64 {
    return _poseidon4_hash_to_u64_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `u128`.
export fn hash_scalar_to_u128_raw(x: scalar) -> u128 {
    return _poseidon4_hash_to_u128_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `i8`.
export fn hash_scalar_to_i8_raw(x: scalar) -> i8 {
    return _poseidon4_hash_to_i8_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `i16`.
export fn hash_scalar_to_i16_raw(x: scalar) -> i16 {
    return _poseidon4_hash_to_i16_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `i32`.
export fn hash_scalar_to_i32_raw(x: scalar) -> i32 {
    return _poseidon4_hash_to_i32_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `i64`.
export fn hash_scalar_to_i64_raw(x: scalar) -> i64 {
    return _poseidon4_hash_to_i64_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `i128`.
export fn hash_scalar_to_i128_raw(x: scalar) -> i128 {
    return _poseidon4_hash_to_i128_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `address`.
export fn hash_address_to_address_raw(x: address) -> address {
    return _poseidon4_hash_to_address_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `field`.
export fn hash_address_to_field_raw(x: address) -> field {
    return _poseidon4_hash_to_field_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `group`.
export fn hash_address_to_group_raw(x: address) -> group {
    return _poseidon4_hash_to_group_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `scalar`.
export fn hash_address_to_scalar_raw(x: address) -> scalar {
    return _poseidon4_hash_to_scalar_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `u8`.
export fn hash_address_to_u8_raw(x: address) -> u8 {
    return _poseidon4_hash_to_u8_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `u16`.
export fn hash_address_to_u16_raw(x: address) -> u16 {
    return _poseidon4_hash_to_u16_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `u32`.
export fn hash_address_to_u32_raw(x: address) -> u32 {
    return _poseidon4_hash_to_u32_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `u64`.
export fn hash_address_to_u64_raw(x: address) -> u64 {
    return _poseidon4_hash_to_u64_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `u128`.
export fn hash_address_to_u128_raw(x: address) -> u128 {
    return _poseidon4_hash_to_u128_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `i8`.
export fn hash_address_to_i8_raw(x: address) -> i8 {
    return _poseidon4_hash_to_i8_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `i16`.
export fn hash_address_to_i16_raw(x: address) -> i16 {
    return _poseidon4_hash_to_i16_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `i32`.
export fn hash_address_to_i32_raw(x: address) -> i32 {
    return _poseidon4_hash_to_i32_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `i64`.
export fn hash_address_to_i64_raw(x: address) -> i64 {
    return _poseidon4_hash_to_i64_raw(x);
}

// Hashes `x` with Poseidon-4 using the raw bit encoding of the input and returns the digest as `i128`.
export fn hash_address_to_i128_raw(x: address) -> i128 {
    return _poseidon4_hash_to_i128_raw(x);
}

