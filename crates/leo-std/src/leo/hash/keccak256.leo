// Keccak-256 hashing.
//
// Keccak is the original sponge construction from which SHA-3 was
// derived. The output bit-width matches the suffix: Keccak-256 produces a
// 256 digest. Use Keccak for interop with Ethereum and other
// ecosystems that standardized on the pre-FIPS variant; for new code that
// needs a NIST-standard hash, prefer the `std::hash::sha3_*` family.
//
// # Variants
//
// Plain "to_<T>" functions return the digest reinterpreted as the named
// output type: `field`, an integer with truncation to its bit width,
// a curve element, or an Aleo address.
//
// `_raw` functions hash the input's native bit pattern directly and
// require the input's bit width to be a multiple of 8 (so the byte-oriented
// sponge can absorb it without padding decisions); only integer inputs and
// arrays of bytes satisfy this. The non-raw variants prepend a type tag
// and accept any input value.
//
// `_to_bits` and `_to_bits_raw` return the digest as a `[bool; N]`
// of the full output width. They also require byte-aligned input.

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `address`.
fn hash_bool_to_address(x: bool) -> address {
    return _keccak256_hash_to_address(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `field`.
fn hash_bool_to_field(x: bool) -> field {
    return _keccak256_hash_to_field(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `group`.
fn hash_bool_to_group(x: bool) -> group {
    return _keccak256_hash_to_group(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `scalar`.
fn hash_bool_to_scalar(x: bool) -> scalar {
    return _keccak256_hash_to_scalar(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `u8`.
fn hash_bool_to_u8(x: bool) -> u8 {
    return _keccak256_hash_to_u8(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `u16`.
fn hash_bool_to_u16(x: bool) -> u16 {
    return _keccak256_hash_to_u16(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `u32`.
fn hash_bool_to_u32(x: bool) -> u32 {
    return _keccak256_hash_to_u32(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `u64`.
fn hash_bool_to_u64(x: bool) -> u64 {
    return _keccak256_hash_to_u64(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `u128`.
fn hash_bool_to_u128(x: bool) -> u128 {
    return _keccak256_hash_to_u128(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `i8`.
fn hash_bool_to_i8(x: bool) -> i8 {
    return _keccak256_hash_to_i8(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `i16`.
fn hash_bool_to_i16(x: bool) -> i16 {
    return _keccak256_hash_to_i16(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `i32`.
fn hash_bool_to_i32(x: bool) -> i32 {
    return _keccak256_hash_to_i32(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `i64`.
fn hash_bool_to_i64(x: bool) -> i64 {
    return _keccak256_hash_to_i64(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `i128`.
fn hash_bool_to_i128(x: bool) -> i128 {
    return _keccak256_hash_to_i128(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `address`.
fn hash_u8_to_address(x: u8) -> address {
    return _keccak256_hash_to_address(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `field`.
fn hash_u8_to_field(x: u8) -> field {
    return _keccak256_hash_to_field(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `group`.
fn hash_u8_to_group(x: u8) -> group {
    return _keccak256_hash_to_group(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `scalar`.
fn hash_u8_to_scalar(x: u8) -> scalar {
    return _keccak256_hash_to_scalar(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `u8`.
fn hash_u8_to_u8(x: u8) -> u8 {
    return _keccak256_hash_to_u8(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `u16`.
fn hash_u8_to_u16(x: u8) -> u16 {
    return _keccak256_hash_to_u16(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `u32`.
fn hash_u8_to_u32(x: u8) -> u32 {
    return _keccak256_hash_to_u32(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `u64`.
fn hash_u8_to_u64(x: u8) -> u64 {
    return _keccak256_hash_to_u64(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `u128`.
fn hash_u8_to_u128(x: u8) -> u128 {
    return _keccak256_hash_to_u128(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `i8`.
fn hash_u8_to_i8(x: u8) -> i8 {
    return _keccak256_hash_to_i8(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `i16`.
fn hash_u8_to_i16(x: u8) -> i16 {
    return _keccak256_hash_to_i16(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `i32`.
fn hash_u8_to_i32(x: u8) -> i32 {
    return _keccak256_hash_to_i32(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `i64`.
fn hash_u8_to_i64(x: u8) -> i64 {
    return _keccak256_hash_to_i64(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `i128`.
fn hash_u8_to_i128(x: u8) -> i128 {
    return _keccak256_hash_to_i128(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `address`.
fn hash_u16_to_address(x: u16) -> address {
    return _keccak256_hash_to_address(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `field`.
fn hash_u16_to_field(x: u16) -> field {
    return _keccak256_hash_to_field(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `group`.
fn hash_u16_to_group(x: u16) -> group {
    return _keccak256_hash_to_group(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `scalar`.
fn hash_u16_to_scalar(x: u16) -> scalar {
    return _keccak256_hash_to_scalar(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `u8`.
fn hash_u16_to_u8(x: u16) -> u8 {
    return _keccak256_hash_to_u8(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `u16`.
fn hash_u16_to_u16(x: u16) -> u16 {
    return _keccak256_hash_to_u16(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `u32`.
fn hash_u16_to_u32(x: u16) -> u32 {
    return _keccak256_hash_to_u32(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `u64`.
fn hash_u16_to_u64(x: u16) -> u64 {
    return _keccak256_hash_to_u64(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `u128`.
fn hash_u16_to_u128(x: u16) -> u128 {
    return _keccak256_hash_to_u128(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `i8`.
fn hash_u16_to_i8(x: u16) -> i8 {
    return _keccak256_hash_to_i8(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `i16`.
fn hash_u16_to_i16(x: u16) -> i16 {
    return _keccak256_hash_to_i16(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `i32`.
fn hash_u16_to_i32(x: u16) -> i32 {
    return _keccak256_hash_to_i32(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `i64`.
fn hash_u16_to_i64(x: u16) -> i64 {
    return _keccak256_hash_to_i64(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `i128`.
fn hash_u16_to_i128(x: u16) -> i128 {
    return _keccak256_hash_to_i128(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `address`.
fn hash_u32_to_address(x: u32) -> address {
    return _keccak256_hash_to_address(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `field`.
fn hash_u32_to_field(x: u32) -> field {
    return _keccak256_hash_to_field(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `group`.
fn hash_u32_to_group(x: u32) -> group {
    return _keccak256_hash_to_group(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `scalar`.
fn hash_u32_to_scalar(x: u32) -> scalar {
    return _keccak256_hash_to_scalar(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `u8`.
fn hash_u32_to_u8(x: u32) -> u8 {
    return _keccak256_hash_to_u8(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `u16`.
fn hash_u32_to_u16(x: u32) -> u16 {
    return _keccak256_hash_to_u16(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `u32`.
fn hash_u32_to_u32(x: u32) -> u32 {
    return _keccak256_hash_to_u32(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `u64`.
fn hash_u32_to_u64(x: u32) -> u64 {
    return _keccak256_hash_to_u64(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `u128`.
fn hash_u32_to_u128(x: u32) -> u128 {
    return _keccak256_hash_to_u128(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `i8`.
fn hash_u32_to_i8(x: u32) -> i8 {
    return _keccak256_hash_to_i8(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `i16`.
fn hash_u32_to_i16(x: u32) -> i16 {
    return _keccak256_hash_to_i16(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `i32`.
fn hash_u32_to_i32(x: u32) -> i32 {
    return _keccak256_hash_to_i32(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `i64`.
fn hash_u32_to_i64(x: u32) -> i64 {
    return _keccak256_hash_to_i64(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `i128`.
fn hash_u32_to_i128(x: u32) -> i128 {
    return _keccak256_hash_to_i128(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `address`.
fn hash_u64_to_address(x: u64) -> address {
    return _keccak256_hash_to_address(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `field`.
fn hash_u64_to_field(x: u64) -> field {
    return _keccak256_hash_to_field(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `group`.
fn hash_u64_to_group(x: u64) -> group {
    return _keccak256_hash_to_group(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `scalar`.
fn hash_u64_to_scalar(x: u64) -> scalar {
    return _keccak256_hash_to_scalar(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `u8`.
fn hash_u64_to_u8(x: u64) -> u8 {
    return _keccak256_hash_to_u8(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `u16`.
fn hash_u64_to_u16(x: u64) -> u16 {
    return _keccak256_hash_to_u16(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `u32`.
fn hash_u64_to_u32(x: u64) -> u32 {
    return _keccak256_hash_to_u32(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `u64`.
fn hash_u64_to_u64(x: u64) -> u64 {
    return _keccak256_hash_to_u64(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `u128`.
fn hash_u64_to_u128(x: u64) -> u128 {
    return _keccak256_hash_to_u128(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `i8`.
fn hash_u64_to_i8(x: u64) -> i8 {
    return _keccak256_hash_to_i8(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `i16`.
fn hash_u64_to_i16(x: u64) -> i16 {
    return _keccak256_hash_to_i16(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `i32`.
fn hash_u64_to_i32(x: u64) -> i32 {
    return _keccak256_hash_to_i32(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `i64`.
fn hash_u64_to_i64(x: u64) -> i64 {
    return _keccak256_hash_to_i64(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `i128`.
fn hash_u64_to_i128(x: u64) -> i128 {
    return _keccak256_hash_to_i128(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `address`.
fn hash_u128_to_address(x: u128) -> address {
    return _keccak256_hash_to_address(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `field`.
fn hash_u128_to_field(x: u128) -> field {
    return _keccak256_hash_to_field(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `group`.
fn hash_u128_to_group(x: u128) -> group {
    return _keccak256_hash_to_group(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `scalar`.
fn hash_u128_to_scalar(x: u128) -> scalar {
    return _keccak256_hash_to_scalar(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `u8`.
fn hash_u128_to_u8(x: u128) -> u8 {
    return _keccak256_hash_to_u8(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `u16`.
fn hash_u128_to_u16(x: u128) -> u16 {
    return _keccak256_hash_to_u16(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `u32`.
fn hash_u128_to_u32(x: u128) -> u32 {
    return _keccak256_hash_to_u32(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `u64`.
fn hash_u128_to_u64(x: u128) -> u64 {
    return _keccak256_hash_to_u64(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `u128`.
fn hash_u128_to_u128(x: u128) -> u128 {
    return _keccak256_hash_to_u128(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `i8`.
fn hash_u128_to_i8(x: u128) -> i8 {
    return _keccak256_hash_to_i8(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `i16`.
fn hash_u128_to_i16(x: u128) -> i16 {
    return _keccak256_hash_to_i16(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `i32`.
fn hash_u128_to_i32(x: u128) -> i32 {
    return _keccak256_hash_to_i32(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `i64`.
fn hash_u128_to_i64(x: u128) -> i64 {
    return _keccak256_hash_to_i64(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `i128`.
fn hash_u128_to_i128(x: u128) -> i128 {
    return _keccak256_hash_to_i128(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `address`.
fn hash_i8_to_address(x: i8) -> address {
    return _keccak256_hash_to_address(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `field`.
fn hash_i8_to_field(x: i8) -> field {
    return _keccak256_hash_to_field(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `group`.
fn hash_i8_to_group(x: i8) -> group {
    return _keccak256_hash_to_group(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `scalar`.
fn hash_i8_to_scalar(x: i8) -> scalar {
    return _keccak256_hash_to_scalar(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `u8`.
fn hash_i8_to_u8(x: i8) -> u8 {
    return _keccak256_hash_to_u8(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `u16`.
fn hash_i8_to_u16(x: i8) -> u16 {
    return _keccak256_hash_to_u16(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `u32`.
fn hash_i8_to_u32(x: i8) -> u32 {
    return _keccak256_hash_to_u32(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `u64`.
fn hash_i8_to_u64(x: i8) -> u64 {
    return _keccak256_hash_to_u64(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `u128`.
fn hash_i8_to_u128(x: i8) -> u128 {
    return _keccak256_hash_to_u128(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `i8`.
fn hash_i8_to_i8(x: i8) -> i8 {
    return _keccak256_hash_to_i8(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `i16`.
fn hash_i8_to_i16(x: i8) -> i16 {
    return _keccak256_hash_to_i16(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `i32`.
fn hash_i8_to_i32(x: i8) -> i32 {
    return _keccak256_hash_to_i32(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `i64`.
fn hash_i8_to_i64(x: i8) -> i64 {
    return _keccak256_hash_to_i64(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `i128`.
fn hash_i8_to_i128(x: i8) -> i128 {
    return _keccak256_hash_to_i128(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `address`.
fn hash_i16_to_address(x: i16) -> address {
    return _keccak256_hash_to_address(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `field`.
fn hash_i16_to_field(x: i16) -> field {
    return _keccak256_hash_to_field(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `group`.
fn hash_i16_to_group(x: i16) -> group {
    return _keccak256_hash_to_group(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `scalar`.
fn hash_i16_to_scalar(x: i16) -> scalar {
    return _keccak256_hash_to_scalar(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `u8`.
fn hash_i16_to_u8(x: i16) -> u8 {
    return _keccak256_hash_to_u8(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `u16`.
fn hash_i16_to_u16(x: i16) -> u16 {
    return _keccak256_hash_to_u16(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `u32`.
fn hash_i16_to_u32(x: i16) -> u32 {
    return _keccak256_hash_to_u32(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `u64`.
fn hash_i16_to_u64(x: i16) -> u64 {
    return _keccak256_hash_to_u64(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `u128`.
fn hash_i16_to_u128(x: i16) -> u128 {
    return _keccak256_hash_to_u128(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `i8`.
fn hash_i16_to_i8(x: i16) -> i8 {
    return _keccak256_hash_to_i8(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `i16`.
fn hash_i16_to_i16(x: i16) -> i16 {
    return _keccak256_hash_to_i16(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `i32`.
fn hash_i16_to_i32(x: i16) -> i32 {
    return _keccak256_hash_to_i32(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `i64`.
fn hash_i16_to_i64(x: i16) -> i64 {
    return _keccak256_hash_to_i64(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `i128`.
fn hash_i16_to_i128(x: i16) -> i128 {
    return _keccak256_hash_to_i128(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `address`.
fn hash_i32_to_address(x: i32) -> address {
    return _keccak256_hash_to_address(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `field`.
fn hash_i32_to_field(x: i32) -> field {
    return _keccak256_hash_to_field(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `group`.
fn hash_i32_to_group(x: i32) -> group {
    return _keccak256_hash_to_group(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `scalar`.
fn hash_i32_to_scalar(x: i32) -> scalar {
    return _keccak256_hash_to_scalar(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `u8`.
fn hash_i32_to_u8(x: i32) -> u8 {
    return _keccak256_hash_to_u8(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `u16`.
fn hash_i32_to_u16(x: i32) -> u16 {
    return _keccak256_hash_to_u16(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `u32`.
fn hash_i32_to_u32(x: i32) -> u32 {
    return _keccak256_hash_to_u32(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `u64`.
fn hash_i32_to_u64(x: i32) -> u64 {
    return _keccak256_hash_to_u64(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `u128`.
fn hash_i32_to_u128(x: i32) -> u128 {
    return _keccak256_hash_to_u128(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `i8`.
fn hash_i32_to_i8(x: i32) -> i8 {
    return _keccak256_hash_to_i8(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `i16`.
fn hash_i32_to_i16(x: i32) -> i16 {
    return _keccak256_hash_to_i16(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `i32`.
fn hash_i32_to_i32(x: i32) -> i32 {
    return _keccak256_hash_to_i32(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `i64`.
fn hash_i32_to_i64(x: i32) -> i64 {
    return _keccak256_hash_to_i64(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `i128`.
fn hash_i32_to_i128(x: i32) -> i128 {
    return _keccak256_hash_to_i128(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `address`.
fn hash_i64_to_address(x: i64) -> address {
    return _keccak256_hash_to_address(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `field`.
fn hash_i64_to_field(x: i64) -> field {
    return _keccak256_hash_to_field(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `group`.
fn hash_i64_to_group(x: i64) -> group {
    return _keccak256_hash_to_group(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `scalar`.
fn hash_i64_to_scalar(x: i64) -> scalar {
    return _keccak256_hash_to_scalar(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `u8`.
fn hash_i64_to_u8(x: i64) -> u8 {
    return _keccak256_hash_to_u8(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `u16`.
fn hash_i64_to_u16(x: i64) -> u16 {
    return _keccak256_hash_to_u16(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `u32`.
fn hash_i64_to_u32(x: i64) -> u32 {
    return _keccak256_hash_to_u32(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `u64`.
fn hash_i64_to_u64(x: i64) -> u64 {
    return _keccak256_hash_to_u64(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `u128`.
fn hash_i64_to_u128(x: i64) -> u128 {
    return _keccak256_hash_to_u128(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `i8`.
fn hash_i64_to_i8(x: i64) -> i8 {
    return _keccak256_hash_to_i8(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `i16`.
fn hash_i64_to_i16(x: i64) -> i16 {
    return _keccak256_hash_to_i16(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `i32`.
fn hash_i64_to_i32(x: i64) -> i32 {
    return _keccak256_hash_to_i32(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `i64`.
fn hash_i64_to_i64(x: i64) -> i64 {
    return _keccak256_hash_to_i64(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `i128`.
fn hash_i64_to_i128(x: i64) -> i128 {
    return _keccak256_hash_to_i128(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `address`.
fn hash_i128_to_address(x: i128) -> address {
    return _keccak256_hash_to_address(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `field`.
fn hash_i128_to_field(x: i128) -> field {
    return _keccak256_hash_to_field(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `group`.
fn hash_i128_to_group(x: i128) -> group {
    return _keccak256_hash_to_group(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `scalar`.
fn hash_i128_to_scalar(x: i128) -> scalar {
    return _keccak256_hash_to_scalar(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `u8`.
fn hash_i128_to_u8(x: i128) -> u8 {
    return _keccak256_hash_to_u8(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `u16`.
fn hash_i128_to_u16(x: i128) -> u16 {
    return _keccak256_hash_to_u16(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `u32`.
fn hash_i128_to_u32(x: i128) -> u32 {
    return _keccak256_hash_to_u32(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `u64`.
fn hash_i128_to_u64(x: i128) -> u64 {
    return _keccak256_hash_to_u64(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `u128`.
fn hash_i128_to_u128(x: i128) -> u128 {
    return _keccak256_hash_to_u128(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `i8`.
fn hash_i128_to_i8(x: i128) -> i8 {
    return _keccak256_hash_to_i8(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `i16`.
fn hash_i128_to_i16(x: i128) -> i16 {
    return _keccak256_hash_to_i16(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `i32`.
fn hash_i128_to_i32(x: i128) -> i32 {
    return _keccak256_hash_to_i32(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `i64`.
fn hash_i128_to_i64(x: i128) -> i64 {
    return _keccak256_hash_to_i64(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `i128`.
fn hash_i128_to_i128(x: i128) -> i128 {
    return _keccak256_hash_to_i128(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `address`.
fn hash_field_to_address(x: field) -> address {
    return _keccak256_hash_to_address(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `field`.
fn hash_field_to_field(x: field) -> field {
    return _keccak256_hash_to_field(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `group`.
fn hash_field_to_group(x: field) -> group {
    return _keccak256_hash_to_group(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `scalar`.
fn hash_field_to_scalar(x: field) -> scalar {
    return _keccak256_hash_to_scalar(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `u8`.
fn hash_field_to_u8(x: field) -> u8 {
    return _keccak256_hash_to_u8(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `u16`.
fn hash_field_to_u16(x: field) -> u16 {
    return _keccak256_hash_to_u16(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `u32`.
fn hash_field_to_u32(x: field) -> u32 {
    return _keccak256_hash_to_u32(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `u64`.
fn hash_field_to_u64(x: field) -> u64 {
    return _keccak256_hash_to_u64(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `u128`.
fn hash_field_to_u128(x: field) -> u128 {
    return _keccak256_hash_to_u128(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `i8`.
fn hash_field_to_i8(x: field) -> i8 {
    return _keccak256_hash_to_i8(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `i16`.
fn hash_field_to_i16(x: field) -> i16 {
    return _keccak256_hash_to_i16(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `i32`.
fn hash_field_to_i32(x: field) -> i32 {
    return _keccak256_hash_to_i32(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `i64`.
fn hash_field_to_i64(x: field) -> i64 {
    return _keccak256_hash_to_i64(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `i128`.
fn hash_field_to_i128(x: field) -> i128 {
    return _keccak256_hash_to_i128(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `address`.
fn hash_group_to_address(x: group) -> address {
    return _keccak256_hash_to_address(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `field`.
fn hash_group_to_field(x: group) -> field {
    return _keccak256_hash_to_field(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `group`.
fn hash_group_to_group(x: group) -> group {
    return _keccak256_hash_to_group(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `scalar`.
fn hash_group_to_scalar(x: group) -> scalar {
    return _keccak256_hash_to_scalar(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `u8`.
fn hash_group_to_u8(x: group) -> u8 {
    return _keccak256_hash_to_u8(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `u16`.
fn hash_group_to_u16(x: group) -> u16 {
    return _keccak256_hash_to_u16(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `u32`.
fn hash_group_to_u32(x: group) -> u32 {
    return _keccak256_hash_to_u32(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `u64`.
fn hash_group_to_u64(x: group) -> u64 {
    return _keccak256_hash_to_u64(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `u128`.
fn hash_group_to_u128(x: group) -> u128 {
    return _keccak256_hash_to_u128(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `i8`.
fn hash_group_to_i8(x: group) -> i8 {
    return _keccak256_hash_to_i8(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `i16`.
fn hash_group_to_i16(x: group) -> i16 {
    return _keccak256_hash_to_i16(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `i32`.
fn hash_group_to_i32(x: group) -> i32 {
    return _keccak256_hash_to_i32(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `i64`.
fn hash_group_to_i64(x: group) -> i64 {
    return _keccak256_hash_to_i64(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `i128`.
fn hash_group_to_i128(x: group) -> i128 {
    return _keccak256_hash_to_i128(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `address`.
fn hash_scalar_to_address(x: scalar) -> address {
    return _keccak256_hash_to_address(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `field`.
fn hash_scalar_to_field(x: scalar) -> field {
    return _keccak256_hash_to_field(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `group`.
fn hash_scalar_to_group(x: scalar) -> group {
    return _keccak256_hash_to_group(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `scalar`.
fn hash_scalar_to_scalar(x: scalar) -> scalar {
    return _keccak256_hash_to_scalar(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `u8`.
fn hash_scalar_to_u8(x: scalar) -> u8 {
    return _keccak256_hash_to_u8(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `u16`.
fn hash_scalar_to_u16(x: scalar) -> u16 {
    return _keccak256_hash_to_u16(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `u32`.
fn hash_scalar_to_u32(x: scalar) -> u32 {
    return _keccak256_hash_to_u32(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `u64`.
fn hash_scalar_to_u64(x: scalar) -> u64 {
    return _keccak256_hash_to_u64(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `u128`.
fn hash_scalar_to_u128(x: scalar) -> u128 {
    return _keccak256_hash_to_u128(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `i8`.
fn hash_scalar_to_i8(x: scalar) -> i8 {
    return _keccak256_hash_to_i8(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `i16`.
fn hash_scalar_to_i16(x: scalar) -> i16 {
    return _keccak256_hash_to_i16(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `i32`.
fn hash_scalar_to_i32(x: scalar) -> i32 {
    return _keccak256_hash_to_i32(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `i64`.
fn hash_scalar_to_i64(x: scalar) -> i64 {
    return _keccak256_hash_to_i64(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `i128`.
fn hash_scalar_to_i128(x: scalar) -> i128 {
    return _keccak256_hash_to_i128(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `address`.
fn hash_address_to_address(x: address) -> address {
    return _keccak256_hash_to_address(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `field`.
fn hash_address_to_field(x: address) -> field {
    return _keccak256_hash_to_field(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `group`.
fn hash_address_to_group(x: address) -> group {
    return _keccak256_hash_to_group(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `scalar`.
fn hash_address_to_scalar(x: address) -> scalar {
    return _keccak256_hash_to_scalar(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `u8`.
fn hash_address_to_u8(x: address) -> u8 {
    return _keccak256_hash_to_u8(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `u16`.
fn hash_address_to_u16(x: address) -> u16 {
    return _keccak256_hash_to_u16(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `u32`.
fn hash_address_to_u32(x: address) -> u32 {
    return _keccak256_hash_to_u32(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `u64`.
fn hash_address_to_u64(x: address) -> u64 {
    return _keccak256_hash_to_u64(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `u128`.
fn hash_address_to_u128(x: address) -> u128 {
    return _keccak256_hash_to_u128(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `i8`.
fn hash_address_to_i8(x: address) -> i8 {
    return _keccak256_hash_to_i8(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `i16`.
fn hash_address_to_i16(x: address) -> i16 {
    return _keccak256_hash_to_i16(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `i32`.
fn hash_address_to_i32(x: address) -> i32 {
    return _keccak256_hash_to_i32(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `i64`.
fn hash_address_to_i64(x: address) -> i64 {
    return _keccak256_hash_to_i64(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the digest as `i128`.
fn hash_address_to_i128(x: address) -> i128 {
    return _keccak256_hash_to_i128(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `address`.
fn hash_u8_to_address_raw(x: u8) -> address {
    return _keccak256_hash_to_address_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `field`.
fn hash_u8_to_field_raw(x: u8) -> field {
    return _keccak256_hash_to_field_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `group`.
fn hash_u8_to_group_raw(x: u8) -> group {
    return _keccak256_hash_to_group_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `scalar`.
fn hash_u8_to_scalar_raw(x: u8) -> scalar {
    return _keccak256_hash_to_scalar_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `u8`.
fn hash_u8_to_u8_raw(x: u8) -> u8 {
    return _keccak256_hash_to_u8_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `u16`.
fn hash_u8_to_u16_raw(x: u8) -> u16 {
    return _keccak256_hash_to_u16_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `u32`.
fn hash_u8_to_u32_raw(x: u8) -> u32 {
    return _keccak256_hash_to_u32_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `u64`.
fn hash_u8_to_u64_raw(x: u8) -> u64 {
    return _keccak256_hash_to_u64_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `u128`.
fn hash_u8_to_u128_raw(x: u8) -> u128 {
    return _keccak256_hash_to_u128_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `i8`.
fn hash_u8_to_i8_raw(x: u8) -> i8 {
    return _keccak256_hash_to_i8_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `i16`.
fn hash_u8_to_i16_raw(x: u8) -> i16 {
    return _keccak256_hash_to_i16_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `i32`.
fn hash_u8_to_i32_raw(x: u8) -> i32 {
    return _keccak256_hash_to_i32_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `i64`.
fn hash_u8_to_i64_raw(x: u8) -> i64 {
    return _keccak256_hash_to_i64_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `i128`.
fn hash_u8_to_i128_raw(x: u8) -> i128 {
    return _keccak256_hash_to_i128_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `address`.
fn hash_u16_to_address_raw(x: u16) -> address {
    return _keccak256_hash_to_address_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `field`.
fn hash_u16_to_field_raw(x: u16) -> field {
    return _keccak256_hash_to_field_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `group`.
fn hash_u16_to_group_raw(x: u16) -> group {
    return _keccak256_hash_to_group_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `scalar`.
fn hash_u16_to_scalar_raw(x: u16) -> scalar {
    return _keccak256_hash_to_scalar_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `u8`.
fn hash_u16_to_u8_raw(x: u16) -> u8 {
    return _keccak256_hash_to_u8_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `u16`.
fn hash_u16_to_u16_raw(x: u16) -> u16 {
    return _keccak256_hash_to_u16_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `u32`.
fn hash_u16_to_u32_raw(x: u16) -> u32 {
    return _keccak256_hash_to_u32_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `u64`.
fn hash_u16_to_u64_raw(x: u16) -> u64 {
    return _keccak256_hash_to_u64_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `u128`.
fn hash_u16_to_u128_raw(x: u16) -> u128 {
    return _keccak256_hash_to_u128_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `i8`.
fn hash_u16_to_i8_raw(x: u16) -> i8 {
    return _keccak256_hash_to_i8_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `i16`.
fn hash_u16_to_i16_raw(x: u16) -> i16 {
    return _keccak256_hash_to_i16_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `i32`.
fn hash_u16_to_i32_raw(x: u16) -> i32 {
    return _keccak256_hash_to_i32_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `i64`.
fn hash_u16_to_i64_raw(x: u16) -> i64 {
    return _keccak256_hash_to_i64_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `i128`.
fn hash_u16_to_i128_raw(x: u16) -> i128 {
    return _keccak256_hash_to_i128_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `address`.
fn hash_u32_to_address_raw(x: u32) -> address {
    return _keccak256_hash_to_address_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `field`.
fn hash_u32_to_field_raw(x: u32) -> field {
    return _keccak256_hash_to_field_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `group`.
fn hash_u32_to_group_raw(x: u32) -> group {
    return _keccak256_hash_to_group_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `scalar`.
fn hash_u32_to_scalar_raw(x: u32) -> scalar {
    return _keccak256_hash_to_scalar_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `u8`.
fn hash_u32_to_u8_raw(x: u32) -> u8 {
    return _keccak256_hash_to_u8_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `u16`.
fn hash_u32_to_u16_raw(x: u32) -> u16 {
    return _keccak256_hash_to_u16_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `u32`.
fn hash_u32_to_u32_raw(x: u32) -> u32 {
    return _keccak256_hash_to_u32_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `u64`.
fn hash_u32_to_u64_raw(x: u32) -> u64 {
    return _keccak256_hash_to_u64_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `u128`.
fn hash_u32_to_u128_raw(x: u32) -> u128 {
    return _keccak256_hash_to_u128_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `i8`.
fn hash_u32_to_i8_raw(x: u32) -> i8 {
    return _keccak256_hash_to_i8_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `i16`.
fn hash_u32_to_i16_raw(x: u32) -> i16 {
    return _keccak256_hash_to_i16_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `i32`.
fn hash_u32_to_i32_raw(x: u32) -> i32 {
    return _keccak256_hash_to_i32_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `i64`.
fn hash_u32_to_i64_raw(x: u32) -> i64 {
    return _keccak256_hash_to_i64_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `i128`.
fn hash_u32_to_i128_raw(x: u32) -> i128 {
    return _keccak256_hash_to_i128_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `address`.
fn hash_u64_to_address_raw(x: u64) -> address {
    return _keccak256_hash_to_address_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `field`.
fn hash_u64_to_field_raw(x: u64) -> field {
    return _keccak256_hash_to_field_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `group`.
fn hash_u64_to_group_raw(x: u64) -> group {
    return _keccak256_hash_to_group_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `scalar`.
fn hash_u64_to_scalar_raw(x: u64) -> scalar {
    return _keccak256_hash_to_scalar_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `u8`.
fn hash_u64_to_u8_raw(x: u64) -> u8 {
    return _keccak256_hash_to_u8_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `u16`.
fn hash_u64_to_u16_raw(x: u64) -> u16 {
    return _keccak256_hash_to_u16_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `u32`.
fn hash_u64_to_u32_raw(x: u64) -> u32 {
    return _keccak256_hash_to_u32_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `u64`.
fn hash_u64_to_u64_raw(x: u64) -> u64 {
    return _keccak256_hash_to_u64_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `u128`.
fn hash_u64_to_u128_raw(x: u64) -> u128 {
    return _keccak256_hash_to_u128_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `i8`.
fn hash_u64_to_i8_raw(x: u64) -> i8 {
    return _keccak256_hash_to_i8_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `i16`.
fn hash_u64_to_i16_raw(x: u64) -> i16 {
    return _keccak256_hash_to_i16_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `i32`.
fn hash_u64_to_i32_raw(x: u64) -> i32 {
    return _keccak256_hash_to_i32_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `i64`.
fn hash_u64_to_i64_raw(x: u64) -> i64 {
    return _keccak256_hash_to_i64_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `i128`.
fn hash_u64_to_i128_raw(x: u64) -> i128 {
    return _keccak256_hash_to_i128_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `address`.
fn hash_u128_to_address_raw(x: u128) -> address {
    return _keccak256_hash_to_address_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `field`.
fn hash_u128_to_field_raw(x: u128) -> field {
    return _keccak256_hash_to_field_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `group`.
fn hash_u128_to_group_raw(x: u128) -> group {
    return _keccak256_hash_to_group_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `scalar`.
fn hash_u128_to_scalar_raw(x: u128) -> scalar {
    return _keccak256_hash_to_scalar_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `u8`.
fn hash_u128_to_u8_raw(x: u128) -> u8 {
    return _keccak256_hash_to_u8_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `u16`.
fn hash_u128_to_u16_raw(x: u128) -> u16 {
    return _keccak256_hash_to_u16_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `u32`.
fn hash_u128_to_u32_raw(x: u128) -> u32 {
    return _keccak256_hash_to_u32_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `u64`.
fn hash_u128_to_u64_raw(x: u128) -> u64 {
    return _keccak256_hash_to_u64_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `u128`.
fn hash_u128_to_u128_raw(x: u128) -> u128 {
    return _keccak256_hash_to_u128_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `i8`.
fn hash_u128_to_i8_raw(x: u128) -> i8 {
    return _keccak256_hash_to_i8_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `i16`.
fn hash_u128_to_i16_raw(x: u128) -> i16 {
    return _keccak256_hash_to_i16_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `i32`.
fn hash_u128_to_i32_raw(x: u128) -> i32 {
    return _keccak256_hash_to_i32_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `i64`.
fn hash_u128_to_i64_raw(x: u128) -> i64 {
    return _keccak256_hash_to_i64_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `i128`.
fn hash_u128_to_i128_raw(x: u128) -> i128 {
    return _keccak256_hash_to_i128_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `address`.
fn hash_i8_to_address_raw(x: i8) -> address {
    return _keccak256_hash_to_address_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `field`.
fn hash_i8_to_field_raw(x: i8) -> field {
    return _keccak256_hash_to_field_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `group`.
fn hash_i8_to_group_raw(x: i8) -> group {
    return _keccak256_hash_to_group_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `scalar`.
fn hash_i8_to_scalar_raw(x: i8) -> scalar {
    return _keccak256_hash_to_scalar_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `u8`.
fn hash_i8_to_u8_raw(x: i8) -> u8 {
    return _keccak256_hash_to_u8_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `u16`.
fn hash_i8_to_u16_raw(x: i8) -> u16 {
    return _keccak256_hash_to_u16_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `u32`.
fn hash_i8_to_u32_raw(x: i8) -> u32 {
    return _keccak256_hash_to_u32_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `u64`.
fn hash_i8_to_u64_raw(x: i8) -> u64 {
    return _keccak256_hash_to_u64_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `u128`.
fn hash_i8_to_u128_raw(x: i8) -> u128 {
    return _keccak256_hash_to_u128_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `i8`.
fn hash_i8_to_i8_raw(x: i8) -> i8 {
    return _keccak256_hash_to_i8_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `i16`.
fn hash_i8_to_i16_raw(x: i8) -> i16 {
    return _keccak256_hash_to_i16_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `i32`.
fn hash_i8_to_i32_raw(x: i8) -> i32 {
    return _keccak256_hash_to_i32_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `i64`.
fn hash_i8_to_i64_raw(x: i8) -> i64 {
    return _keccak256_hash_to_i64_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `i128`.
fn hash_i8_to_i128_raw(x: i8) -> i128 {
    return _keccak256_hash_to_i128_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `address`.
fn hash_i16_to_address_raw(x: i16) -> address {
    return _keccak256_hash_to_address_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `field`.
fn hash_i16_to_field_raw(x: i16) -> field {
    return _keccak256_hash_to_field_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `group`.
fn hash_i16_to_group_raw(x: i16) -> group {
    return _keccak256_hash_to_group_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `scalar`.
fn hash_i16_to_scalar_raw(x: i16) -> scalar {
    return _keccak256_hash_to_scalar_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `u8`.
fn hash_i16_to_u8_raw(x: i16) -> u8 {
    return _keccak256_hash_to_u8_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `u16`.
fn hash_i16_to_u16_raw(x: i16) -> u16 {
    return _keccak256_hash_to_u16_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `u32`.
fn hash_i16_to_u32_raw(x: i16) -> u32 {
    return _keccak256_hash_to_u32_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `u64`.
fn hash_i16_to_u64_raw(x: i16) -> u64 {
    return _keccak256_hash_to_u64_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `u128`.
fn hash_i16_to_u128_raw(x: i16) -> u128 {
    return _keccak256_hash_to_u128_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `i8`.
fn hash_i16_to_i8_raw(x: i16) -> i8 {
    return _keccak256_hash_to_i8_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `i16`.
fn hash_i16_to_i16_raw(x: i16) -> i16 {
    return _keccak256_hash_to_i16_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `i32`.
fn hash_i16_to_i32_raw(x: i16) -> i32 {
    return _keccak256_hash_to_i32_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `i64`.
fn hash_i16_to_i64_raw(x: i16) -> i64 {
    return _keccak256_hash_to_i64_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `i128`.
fn hash_i16_to_i128_raw(x: i16) -> i128 {
    return _keccak256_hash_to_i128_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `address`.
fn hash_i32_to_address_raw(x: i32) -> address {
    return _keccak256_hash_to_address_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `field`.
fn hash_i32_to_field_raw(x: i32) -> field {
    return _keccak256_hash_to_field_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `group`.
fn hash_i32_to_group_raw(x: i32) -> group {
    return _keccak256_hash_to_group_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `scalar`.
fn hash_i32_to_scalar_raw(x: i32) -> scalar {
    return _keccak256_hash_to_scalar_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `u8`.
fn hash_i32_to_u8_raw(x: i32) -> u8 {
    return _keccak256_hash_to_u8_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `u16`.
fn hash_i32_to_u16_raw(x: i32) -> u16 {
    return _keccak256_hash_to_u16_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `u32`.
fn hash_i32_to_u32_raw(x: i32) -> u32 {
    return _keccak256_hash_to_u32_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `u64`.
fn hash_i32_to_u64_raw(x: i32) -> u64 {
    return _keccak256_hash_to_u64_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `u128`.
fn hash_i32_to_u128_raw(x: i32) -> u128 {
    return _keccak256_hash_to_u128_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `i8`.
fn hash_i32_to_i8_raw(x: i32) -> i8 {
    return _keccak256_hash_to_i8_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `i16`.
fn hash_i32_to_i16_raw(x: i32) -> i16 {
    return _keccak256_hash_to_i16_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `i32`.
fn hash_i32_to_i32_raw(x: i32) -> i32 {
    return _keccak256_hash_to_i32_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `i64`.
fn hash_i32_to_i64_raw(x: i32) -> i64 {
    return _keccak256_hash_to_i64_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `i128`.
fn hash_i32_to_i128_raw(x: i32) -> i128 {
    return _keccak256_hash_to_i128_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `address`.
fn hash_i64_to_address_raw(x: i64) -> address {
    return _keccak256_hash_to_address_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `field`.
fn hash_i64_to_field_raw(x: i64) -> field {
    return _keccak256_hash_to_field_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `group`.
fn hash_i64_to_group_raw(x: i64) -> group {
    return _keccak256_hash_to_group_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `scalar`.
fn hash_i64_to_scalar_raw(x: i64) -> scalar {
    return _keccak256_hash_to_scalar_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `u8`.
fn hash_i64_to_u8_raw(x: i64) -> u8 {
    return _keccak256_hash_to_u8_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `u16`.
fn hash_i64_to_u16_raw(x: i64) -> u16 {
    return _keccak256_hash_to_u16_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `u32`.
fn hash_i64_to_u32_raw(x: i64) -> u32 {
    return _keccak256_hash_to_u32_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `u64`.
fn hash_i64_to_u64_raw(x: i64) -> u64 {
    return _keccak256_hash_to_u64_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `u128`.
fn hash_i64_to_u128_raw(x: i64) -> u128 {
    return _keccak256_hash_to_u128_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `i8`.
fn hash_i64_to_i8_raw(x: i64) -> i8 {
    return _keccak256_hash_to_i8_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `i16`.
fn hash_i64_to_i16_raw(x: i64) -> i16 {
    return _keccak256_hash_to_i16_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `i32`.
fn hash_i64_to_i32_raw(x: i64) -> i32 {
    return _keccak256_hash_to_i32_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `i64`.
fn hash_i64_to_i64_raw(x: i64) -> i64 {
    return _keccak256_hash_to_i64_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `i128`.
fn hash_i64_to_i128_raw(x: i64) -> i128 {
    return _keccak256_hash_to_i128_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `address`.
fn hash_i128_to_address_raw(x: i128) -> address {
    return _keccak256_hash_to_address_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `field`.
fn hash_i128_to_field_raw(x: i128) -> field {
    return _keccak256_hash_to_field_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `group`.
fn hash_i128_to_group_raw(x: i128) -> group {
    return _keccak256_hash_to_group_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `scalar`.
fn hash_i128_to_scalar_raw(x: i128) -> scalar {
    return _keccak256_hash_to_scalar_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `u8`.
fn hash_i128_to_u8_raw(x: i128) -> u8 {
    return _keccak256_hash_to_u8_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `u16`.
fn hash_i128_to_u16_raw(x: i128) -> u16 {
    return _keccak256_hash_to_u16_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `u32`.
fn hash_i128_to_u32_raw(x: i128) -> u32 {
    return _keccak256_hash_to_u32_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `u64`.
fn hash_i128_to_u64_raw(x: i128) -> u64 {
    return _keccak256_hash_to_u64_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `u128`.
fn hash_i128_to_u128_raw(x: i128) -> u128 {
    return _keccak256_hash_to_u128_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `i8`.
fn hash_i128_to_i8_raw(x: i128) -> i8 {
    return _keccak256_hash_to_i8_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `i16`.
fn hash_i128_to_i16_raw(x: i128) -> i16 {
    return _keccak256_hash_to_i16_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `i32`.
fn hash_i128_to_i32_raw(x: i128) -> i32 {
    return _keccak256_hash_to_i32_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `i64`.
fn hash_i128_to_i64_raw(x: i128) -> i64 {
    return _keccak256_hash_to_i64_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the digest as `i128`.
fn hash_i128_to_i128_raw(x: i128) -> i128 {
    return _keccak256_hash_to_i128_raw(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the full 256-bit digest as a bit array.
fn hash_u8_to_bits(x: u8) -> [bool; 256] {
    return _keccak256_hash_to_bits(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the full 256-bit digest as a bit array.
fn hash_u16_to_bits(x: u16) -> [bool; 256] {
    return _keccak256_hash_to_bits(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the full 256-bit digest as a bit array.
fn hash_u32_to_bits(x: u32) -> [bool; 256] {
    return _keccak256_hash_to_bits(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the full 256-bit digest as a bit array.
fn hash_u64_to_bits(x: u64) -> [bool; 256] {
    return _keccak256_hash_to_bits(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the full 256-bit digest as a bit array.
fn hash_u128_to_bits(x: u128) -> [bool; 256] {
    return _keccak256_hash_to_bits(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the full 256-bit digest as a bit array.
fn hash_i8_to_bits(x: i8) -> [bool; 256] {
    return _keccak256_hash_to_bits(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the full 256-bit digest as a bit array.
fn hash_i16_to_bits(x: i16) -> [bool; 256] {
    return _keccak256_hash_to_bits(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the full 256-bit digest as a bit array.
fn hash_i32_to_bits(x: i32) -> [bool; 256] {
    return _keccak256_hash_to_bits(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the full 256-bit digest as a bit array.
fn hash_i64_to_bits(x: i64) -> [bool; 256] {
    return _keccak256_hash_to_bits(x);
}

// Hashes `x` with Keccak-256 (tagged input encoding) and returns the full 256-bit digest as a bit array.
fn hash_i128_to_bits(x: i128) -> [bool; 256] {
    return _keccak256_hash_to_bits(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the full 256-bit digest as a bit array.
fn hash_u8_to_bits_raw(x: u8) -> [bool; 256] {
    return _keccak256_hash_to_bits_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the full 256-bit digest as a bit array.
fn hash_u16_to_bits_raw(x: u16) -> [bool; 256] {
    return _keccak256_hash_to_bits_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the full 256-bit digest as a bit array.
fn hash_u32_to_bits_raw(x: u32) -> [bool; 256] {
    return _keccak256_hash_to_bits_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the full 256-bit digest as a bit array.
fn hash_u64_to_bits_raw(x: u64) -> [bool; 256] {
    return _keccak256_hash_to_bits_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the full 256-bit digest as a bit array.
fn hash_u128_to_bits_raw(x: u128) -> [bool; 256] {
    return _keccak256_hash_to_bits_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the full 256-bit digest as a bit array.
fn hash_i8_to_bits_raw(x: i8) -> [bool; 256] {
    return _keccak256_hash_to_bits_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the full 256-bit digest as a bit array.
fn hash_i16_to_bits_raw(x: i16) -> [bool; 256] {
    return _keccak256_hash_to_bits_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the full 256-bit digest as a bit array.
fn hash_i32_to_bits_raw(x: i32) -> [bool; 256] {
    return _keccak256_hash_to_bits_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the full 256-bit digest as a bit array.
fn hash_i64_to_bits_raw(x: i64) -> [bool; 256] {
    return _keccak256_hash_to_bits_raw(x);
}

// Hashes `x` with Keccak-256 using the raw bit encoding of the input and returns the full 256-bit digest as a bit array.
fn hash_i128_to_bits_raw(x: i128) -> [bool; 256] {
    return _keccak256_hash_to_bits_raw(x);
}

