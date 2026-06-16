// Pedersen hashing for inputs up to 64 bits.
//
// Pedersen is a homomorphic, collision-resistant hash family that is very
// cheap to prove inside a zk-SNARK. Its capacity is bounded by the input
// size, so this variant accepts only `bool` and integer types of 32 bits
// or fewer. Call sites with wider inputs must use `pedersen128` or a
// BHP/Poseidon variant.
//
// # Variants
//
// Each "to_<T>" function returns the hash interpreted as a value of type
// `T` (field, group, address, integer, or scalar).
//
// `_raw` variants hash the input's bit pattern directly; the non-raw
// variants prepend a type tag so two values of different but bit-equal
// types do not collide.

// Hashes `x` with Pedersen-64 (tagged input encoding) and returns the digest as `address`.
fn hash_bool_to_address(x: bool) -> address {
    return _pedersen64_hash_to_address(x);
}

// Hashes `x` with Pedersen-64 (tagged input encoding) and returns the digest as `field`.
fn hash_bool_to_field(x: bool) -> field {
    return _pedersen64_hash_to_field(x);
}

// Hashes `x` with Pedersen-64 (tagged input encoding) and returns the digest as `group`.
fn hash_bool_to_group(x: bool) -> group {
    return _pedersen64_hash_to_group(x);
}

// Hashes `x` with Pedersen-64 (tagged input encoding) and returns the digest as `scalar`.
fn hash_bool_to_scalar(x: bool) -> scalar {
    return _pedersen64_hash_to_scalar(x);
}

// Hashes `x` with Pedersen-64 (tagged input encoding) and returns the digest as `u8`.
fn hash_bool_to_u8(x: bool) -> u8 {
    return _pedersen64_hash_to_u8(x);
}

// Hashes `x` with Pedersen-64 (tagged input encoding) and returns the digest as `u16`.
fn hash_bool_to_u16(x: bool) -> u16 {
    return _pedersen64_hash_to_u16(x);
}

// Hashes `x` with Pedersen-64 (tagged input encoding) and returns the digest as `u32`.
fn hash_bool_to_u32(x: bool) -> u32 {
    return _pedersen64_hash_to_u32(x);
}

// Hashes `x` with Pedersen-64 (tagged input encoding) and returns the digest as `u64`.
fn hash_bool_to_u64(x: bool) -> u64 {
    return _pedersen64_hash_to_u64(x);
}

// Hashes `x` with Pedersen-64 (tagged input encoding) and returns the digest as `u128`.
fn hash_bool_to_u128(x: bool) -> u128 {
    return _pedersen64_hash_to_u128(x);
}

// Hashes `x` with Pedersen-64 (tagged input encoding) and returns the digest as `i8`.
fn hash_bool_to_i8(x: bool) -> i8 {
    return _pedersen64_hash_to_i8(x);
}

// Hashes `x` with Pedersen-64 (tagged input encoding) and returns the digest as `i16`.
fn hash_bool_to_i16(x: bool) -> i16 {
    return _pedersen64_hash_to_i16(x);
}

// Hashes `x` with Pedersen-64 (tagged input encoding) and returns the digest as `i32`.
fn hash_bool_to_i32(x: bool) -> i32 {
    return _pedersen64_hash_to_i32(x);
}

// Hashes `x` with Pedersen-64 (tagged input encoding) and returns the digest as `i64`.
fn hash_bool_to_i64(x: bool) -> i64 {
    return _pedersen64_hash_to_i64(x);
}

// Hashes `x` with Pedersen-64 (tagged input encoding) and returns the digest as `i128`.
fn hash_bool_to_i128(x: bool) -> i128 {
    return _pedersen64_hash_to_i128(x);
}

// Hashes `x` with Pedersen-64 (tagged input encoding) and returns the digest as `address`.
fn hash_u8_to_address(x: u8) -> address {
    return _pedersen64_hash_to_address(x);
}

// Hashes `x` with Pedersen-64 (tagged input encoding) and returns the digest as `field`.
fn hash_u8_to_field(x: u8) -> field {
    return _pedersen64_hash_to_field(x);
}

// Hashes `x` with Pedersen-64 (tagged input encoding) and returns the digest as `group`.
fn hash_u8_to_group(x: u8) -> group {
    return _pedersen64_hash_to_group(x);
}

// Hashes `x` with Pedersen-64 (tagged input encoding) and returns the digest as `scalar`.
fn hash_u8_to_scalar(x: u8) -> scalar {
    return _pedersen64_hash_to_scalar(x);
}

// Hashes `x` with Pedersen-64 (tagged input encoding) and returns the digest as `u8`.
fn hash_u8_to_u8(x: u8) -> u8 {
    return _pedersen64_hash_to_u8(x);
}

// Hashes `x` with Pedersen-64 (tagged input encoding) and returns the digest as `u16`.
fn hash_u8_to_u16(x: u8) -> u16 {
    return _pedersen64_hash_to_u16(x);
}

// Hashes `x` with Pedersen-64 (tagged input encoding) and returns the digest as `u32`.
fn hash_u8_to_u32(x: u8) -> u32 {
    return _pedersen64_hash_to_u32(x);
}

// Hashes `x` with Pedersen-64 (tagged input encoding) and returns the digest as `u64`.
fn hash_u8_to_u64(x: u8) -> u64 {
    return _pedersen64_hash_to_u64(x);
}

// Hashes `x` with Pedersen-64 (tagged input encoding) and returns the digest as `u128`.
fn hash_u8_to_u128(x: u8) -> u128 {
    return _pedersen64_hash_to_u128(x);
}

// Hashes `x` with Pedersen-64 (tagged input encoding) and returns the digest as `i8`.
fn hash_u8_to_i8(x: u8) -> i8 {
    return _pedersen64_hash_to_i8(x);
}

// Hashes `x` with Pedersen-64 (tagged input encoding) and returns the digest as `i16`.
fn hash_u8_to_i16(x: u8) -> i16 {
    return _pedersen64_hash_to_i16(x);
}

// Hashes `x` with Pedersen-64 (tagged input encoding) and returns the digest as `i32`.
fn hash_u8_to_i32(x: u8) -> i32 {
    return _pedersen64_hash_to_i32(x);
}

// Hashes `x` with Pedersen-64 (tagged input encoding) and returns the digest as `i64`.
fn hash_u8_to_i64(x: u8) -> i64 {
    return _pedersen64_hash_to_i64(x);
}

// Hashes `x` with Pedersen-64 (tagged input encoding) and returns the digest as `i128`.
fn hash_u8_to_i128(x: u8) -> i128 {
    return _pedersen64_hash_to_i128(x);
}

// Hashes `x` with Pedersen-64 (tagged input encoding) and returns the digest as `address`.
fn hash_u16_to_address(x: u16) -> address {
    return _pedersen64_hash_to_address(x);
}

// Hashes `x` with Pedersen-64 (tagged input encoding) and returns the digest as `field`.
fn hash_u16_to_field(x: u16) -> field {
    return _pedersen64_hash_to_field(x);
}

// Hashes `x` with Pedersen-64 (tagged input encoding) and returns the digest as `group`.
fn hash_u16_to_group(x: u16) -> group {
    return _pedersen64_hash_to_group(x);
}

// Hashes `x` with Pedersen-64 (tagged input encoding) and returns the digest as `scalar`.
fn hash_u16_to_scalar(x: u16) -> scalar {
    return _pedersen64_hash_to_scalar(x);
}

// Hashes `x` with Pedersen-64 (tagged input encoding) and returns the digest as `u8`.
fn hash_u16_to_u8(x: u16) -> u8 {
    return _pedersen64_hash_to_u8(x);
}

// Hashes `x` with Pedersen-64 (tagged input encoding) and returns the digest as `u16`.
fn hash_u16_to_u16(x: u16) -> u16 {
    return _pedersen64_hash_to_u16(x);
}

// Hashes `x` with Pedersen-64 (tagged input encoding) and returns the digest as `u32`.
fn hash_u16_to_u32(x: u16) -> u32 {
    return _pedersen64_hash_to_u32(x);
}

// Hashes `x` with Pedersen-64 (tagged input encoding) and returns the digest as `u64`.
fn hash_u16_to_u64(x: u16) -> u64 {
    return _pedersen64_hash_to_u64(x);
}

// Hashes `x` with Pedersen-64 (tagged input encoding) and returns the digest as `u128`.
fn hash_u16_to_u128(x: u16) -> u128 {
    return _pedersen64_hash_to_u128(x);
}

// Hashes `x` with Pedersen-64 (tagged input encoding) and returns the digest as `i8`.
fn hash_u16_to_i8(x: u16) -> i8 {
    return _pedersen64_hash_to_i8(x);
}

// Hashes `x` with Pedersen-64 (tagged input encoding) and returns the digest as `i16`.
fn hash_u16_to_i16(x: u16) -> i16 {
    return _pedersen64_hash_to_i16(x);
}

// Hashes `x` with Pedersen-64 (tagged input encoding) and returns the digest as `i32`.
fn hash_u16_to_i32(x: u16) -> i32 {
    return _pedersen64_hash_to_i32(x);
}

// Hashes `x` with Pedersen-64 (tagged input encoding) and returns the digest as `i64`.
fn hash_u16_to_i64(x: u16) -> i64 {
    return _pedersen64_hash_to_i64(x);
}

// Hashes `x` with Pedersen-64 (tagged input encoding) and returns the digest as `i128`.
fn hash_u16_to_i128(x: u16) -> i128 {
    return _pedersen64_hash_to_i128(x);
}

// Hashes `x` with Pedersen-64 (tagged input encoding) and returns the digest as `address`.
fn hash_u32_to_address(x: u32) -> address {
    return _pedersen64_hash_to_address(x);
}

// Hashes `x` with Pedersen-64 (tagged input encoding) and returns the digest as `field`.
fn hash_u32_to_field(x: u32) -> field {
    return _pedersen64_hash_to_field(x);
}

// Hashes `x` with Pedersen-64 (tagged input encoding) and returns the digest as `group`.
fn hash_u32_to_group(x: u32) -> group {
    return _pedersen64_hash_to_group(x);
}

// Hashes `x` with Pedersen-64 (tagged input encoding) and returns the digest as `scalar`.
fn hash_u32_to_scalar(x: u32) -> scalar {
    return _pedersen64_hash_to_scalar(x);
}

// Hashes `x` with Pedersen-64 (tagged input encoding) and returns the digest as `u8`.
fn hash_u32_to_u8(x: u32) -> u8 {
    return _pedersen64_hash_to_u8(x);
}

// Hashes `x` with Pedersen-64 (tagged input encoding) and returns the digest as `u16`.
fn hash_u32_to_u16(x: u32) -> u16 {
    return _pedersen64_hash_to_u16(x);
}

// Hashes `x` with Pedersen-64 (tagged input encoding) and returns the digest as `u32`.
fn hash_u32_to_u32(x: u32) -> u32 {
    return _pedersen64_hash_to_u32(x);
}

// Hashes `x` with Pedersen-64 (tagged input encoding) and returns the digest as `u64`.
fn hash_u32_to_u64(x: u32) -> u64 {
    return _pedersen64_hash_to_u64(x);
}

// Hashes `x` with Pedersen-64 (tagged input encoding) and returns the digest as `u128`.
fn hash_u32_to_u128(x: u32) -> u128 {
    return _pedersen64_hash_to_u128(x);
}

// Hashes `x` with Pedersen-64 (tagged input encoding) and returns the digest as `i8`.
fn hash_u32_to_i8(x: u32) -> i8 {
    return _pedersen64_hash_to_i8(x);
}

// Hashes `x` with Pedersen-64 (tagged input encoding) and returns the digest as `i16`.
fn hash_u32_to_i16(x: u32) -> i16 {
    return _pedersen64_hash_to_i16(x);
}

// Hashes `x` with Pedersen-64 (tagged input encoding) and returns the digest as `i32`.
fn hash_u32_to_i32(x: u32) -> i32 {
    return _pedersen64_hash_to_i32(x);
}

// Hashes `x` with Pedersen-64 (tagged input encoding) and returns the digest as `i64`.
fn hash_u32_to_i64(x: u32) -> i64 {
    return _pedersen64_hash_to_i64(x);
}

// Hashes `x` with Pedersen-64 (tagged input encoding) and returns the digest as `i128`.
fn hash_u32_to_i128(x: u32) -> i128 {
    return _pedersen64_hash_to_i128(x);
}

// Hashes `x` with Pedersen-64 (tagged input encoding) and returns the digest as `address`.
fn hash_i8_to_address(x: i8) -> address {
    return _pedersen64_hash_to_address(x);
}

// Hashes `x` with Pedersen-64 (tagged input encoding) and returns the digest as `field`.
fn hash_i8_to_field(x: i8) -> field {
    return _pedersen64_hash_to_field(x);
}

// Hashes `x` with Pedersen-64 (tagged input encoding) and returns the digest as `group`.
fn hash_i8_to_group(x: i8) -> group {
    return _pedersen64_hash_to_group(x);
}

// Hashes `x` with Pedersen-64 (tagged input encoding) and returns the digest as `scalar`.
fn hash_i8_to_scalar(x: i8) -> scalar {
    return _pedersen64_hash_to_scalar(x);
}

// Hashes `x` with Pedersen-64 (tagged input encoding) and returns the digest as `u8`.
fn hash_i8_to_u8(x: i8) -> u8 {
    return _pedersen64_hash_to_u8(x);
}

// Hashes `x` with Pedersen-64 (tagged input encoding) and returns the digest as `u16`.
fn hash_i8_to_u16(x: i8) -> u16 {
    return _pedersen64_hash_to_u16(x);
}

// Hashes `x` with Pedersen-64 (tagged input encoding) and returns the digest as `u32`.
fn hash_i8_to_u32(x: i8) -> u32 {
    return _pedersen64_hash_to_u32(x);
}

// Hashes `x` with Pedersen-64 (tagged input encoding) and returns the digest as `u64`.
fn hash_i8_to_u64(x: i8) -> u64 {
    return _pedersen64_hash_to_u64(x);
}

// Hashes `x` with Pedersen-64 (tagged input encoding) and returns the digest as `u128`.
fn hash_i8_to_u128(x: i8) -> u128 {
    return _pedersen64_hash_to_u128(x);
}

// Hashes `x` with Pedersen-64 (tagged input encoding) and returns the digest as `i8`.
fn hash_i8_to_i8(x: i8) -> i8 {
    return _pedersen64_hash_to_i8(x);
}

// Hashes `x` with Pedersen-64 (tagged input encoding) and returns the digest as `i16`.
fn hash_i8_to_i16(x: i8) -> i16 {
    return _pedersen64_hash_to_i16(x);
}

// Hashes `x` with Pedersen-64 (tagged input encoding) and returns the digest as `i32`.
fn hash_i8_to_i32(x: i8) -> i32 {
    return _pedersen64_hash_to_i32(x);
}

// Hashes `x` with Pedersen-64 (tagged input encoding) and returns the digest as `i64`.
fn hash_i8_to_i64(x: i8) -> i64 {
    return _pedersen64_hash_to_i64(x);
}

// Hashes `x` with Pedersen-64 (tagged input encoding) and returns the digest as `i128`.
fn hash_i8_to_i128(x: i8) -> i128 {
    return _pedersen64_hash_to_i128(x);
}

// Hashes `x` with Pedersen-64 (tagged input encoding) and returns the digest as `address`.
fn hash_i16_to_address(x: i16) -> address {
    return _pedersen64_hash_to_address(x);
}

// Hashes `x` with Pedersen-64 (tagged input encoding) and returns the digest as `field`.
fn hash_i16_to_field(x: i16) -> field {
    return _pedersen64_hash_to_field(x);
}

// Hashes `x` with Pedersen-64 (tagged input encoding) and returns the digest as `group`.
fn hash_i16_to_group(x: i16) -> group {
    return _pedersen64_hash_to_group(x);
}

// Hashes `x` with Pedersen-64 (tagged input encoding) and returns the digest as `scalar`.
fn hash_i16_to_scalar(x: i16) -> scalar {
    return _pedersen64_hash_to_scalar(x);
}

// Hashes `x` with Pedersen-64 (tagged input encoding) and returns the digest as `u8`.
fn hash_i16_to_u8(x: i16) -> u8 {
    return _pedersen64_hash_to_u8(x);
}

// Hashes `x` with Pedersen-64 (tagged input encoding) and returns the digest as `u16`.
fn hash_i16_to_u16(x: i16) -> u16 {
    return _pedersen64_hash_to_u16(x);
}

// Hashes `x` with Pedersen-64 (tagged input encoding) and returns the digest as `u32`.
fn hash_i16_to_u32(x: i16) -> u32 {
    return _pedersen64_hash_to_u32(x);
}

// Hashes `x` with Pedersen-64 (tagged input encoding) and returns the digest as `u64`.
fn hash_i16_to_u64(x: i16) -> u64 {
    return _pedersen64_hash_to_u64(x);
}

// Hashes `x` with Pedersen-64 (tagged input encoding) and returns the digest as `u128`.
fn hash_i16_to_u128(x: i16) -> u128 {
    return _pedersen64_hash_to_u128(x);
}

// Hashes `x` with Pedersen-64 (tagged input encoding) and returns the digest as `i8`.
fn hash_i16_to_i8(x: i16) -> i8 {
    return _pedersen64_hash_to_i8(x);
}

// Hashes `x` with Pedersen-64 (tagged input encoding) and returns the digest as `i16`.
fn hash_i16_to_i16(x: i16) -> i16 {
    return _pedersen64_hash_to_i16(x);
}

// Hashes `x` with Pedersen-64 (tagged input encoding) and returns the digest as `i32`.
fn hash_i16_to_i32(x: i16) -> i32 {
    return _pedersen64_hash_to_i32(x);
}

// Hashes `x` with Pedersen-64 (tagged input encoding) and returns the digest as `i64`.
fn hash_i16_to_i64(x: i16) -> i64 {
    return _pedersen64_hash_to_i64(x);
}

// Hashes `x` with Pedersen-64 (tagged input encoding) and returns the digest as `i128`.
fn hash_i16_to_i128(x: i16) -> i128 {
    return _pedersen64_hash_to_i128(x);
}

// Hashes `x` with Pedersen-64 (tagged input encoding) and returns the digest as `address`.
fn hash_i32_to_address(x: i32) -> address {
    return _pedersen64_hash_to_address(x);
}

// Hashes `x` with Pedersen-64 (tagged input encoding) and returns the digest as `field`.
fn hash_i32_to_field(x: i32) -> field {
    return _pedersen64_hash_to_field(x);
}

// Hashes `x` with Pedersen-64 (tagged input encoding) and returns the digest as `group`.
fn hash_i32_to_group(x: i32) -> group {
    return _pedersen64_hash_to_group(x);
}

// Hashes `x` with Pedersen-64 (tagged input encoding) and returns the digest as `scalar`.
fn hash_i32_to_scalar(x: i32) -> scalar {
    return _pedersen64_hash_to_scalar(x);
}

// Hashes `x` with Pedersen-64 (tagged input encoding) and returns the digest as `u8`.
fn hash_i32_to_u8(x: i32) -> u8 {
    return _pedersen64_hash_to_u8(x);
}

// Hashes `x` with Pedersen-64 (tagged input encoding) and returns the digest as `u16`.
fn hash_i32_to_u16(x: i32) -> u16 {
    return _pedersen64_hash_to_u16(x);
}

// Hashes `x` with Pedersen-64 (tagged input encoding) and returns the digest as `u32`.
fn hash_i32_to_u32(x: i32) -> u32 {
    return _pedersen64_hash_to_u32(x);
}

// Hashes `x` with Pedersen-64 (tagged input encoding) and returns the digest as `u64`.
fn hash_i32_to_u64(x: i32) -> u64 {
    return _pedersen64_hash_to_u64(x);
}

// Hashes `x` with Pedersen-64 (tagged input encoding) and returns the digest as `u128`.
fn hash_i32_to_u128(x: i32) -> u128 {
    return _pedersen64_hash_to_u128(x);
}

// Hashes `x` with Pedersen-64 (tagged input encoding) and returns the digest as `i8`.
fn hash_i32_to_i8(x: i32) -> i8 {
    return _pedersen64_hash_to_i8(x);
}

// Hashes `x` with Pedersen-64 (tagged input encoding) and returns the digest as `i16`.
fn hash_i32_to_i16(x: i32) -> i16 {
    return _pedersen64_hash_to_i16(x);
}

// Hashes `x` with Pedersen-64 (tagged input encoding) and returns the digest as `i32`.
fn hash_i32_to_i32(x: i32) -> i32 {
    return _pedersen64_hash_to_i32(x);
}

// Hashes `x` with Pedersen-64 (tagged input encoding) and returns the digest as `i64`.
fn hash_i32_to_i64(x: i32) -> i64 {
    return _pedersen64_hash_to_i64(x);
}

// Hashes `x` with Pedersen-64 (tagged input encoding) and returns the digest as `i128`.
fn hash_i32_to_i128(x: i32) -> i128 {
    return _pedersen64_hash_to_i128(x);
}

// Hashes `x` with Pedersen-64 using the raw bit encoding of the input and returns the digest as `address`.
fn hash_bool_to_address_raw(x: bool) -> address {
    return _pedersen64_hash_to_address_raw(x);
}

// Hashes `x` with Pedersen-64 using the raw bit encoding of the input and returns the digest as `field`.
fn hash_bool_to_field_raw(x: bool) -> field {
    return _pedersen64_hash_to_field_raw(x);
}

// Hashes `x` with Pedersen-64 using the raw bit encoding of the input and returns the digest as `group`.
fn hash_bool_to_group_raw(x: bool) -> group {
    return _pedersen64_hash_to_group_raw(x);
}

// Hashes `x` with Pedersen-64 using the raw bit encoding of the input and returns the digest as `scalar`.
fn hash_bool_to_scalar_raw(x: bool) -> scalar {
    return _pedersen64_hash_to_scalar_raw(x);
}

// Hashes `x` with Pedersen-64 using the raw bit encoding of the input and returns the digest as `u8`.
fn hash_bool_to_u8_raw(x: bool) -> u8 {
    return _pedersen64_hash_to_u8_raw(x);
}

// Hashes `x` with Pedersen-64 using the raw bit encoding of the input and returns the digest as `u16`.
fn hash_bool_to_u16_raw(x: bool) -> u16 {
    return _pedersen64_hash_to_u16_raw(x);
}

// Hashes `x` with Pedersen-64 using the raw bit encoding of the input and returns the digest as `u32`.
fn hash_bool_to_u32_raw(x: bool) -> u32 {
    return _pedersen64_hash_to_u32_raw(x);
}

// Hashes `x` with Pedersen-64 using the raw bit encoding of the input and returns the digest as `u64`.
fn hash_bool_to_u64_raw(x: bool) -> u64 {
    return _pedersen64_hash_to_u64_raw(x);
}

// Hashes `x` with Pedersen-64 using the raw bit encoding of the input and returns the digest as `u128`.
fn hash_bool_to_u128_raw(x: bool) -> u128 {
    return _pedersen64_hash_to_u128_raw(x);
}

// Hashes `x` with Pedersen-64 using the raw bit encoding of the input and returns the digest as `i8`.
fn hash_bool_to_i8_raw(x: bool) -> i8 {
    return _pedersen64_hash_to_i8_raw(x);
}

// Hashes `x` with Pedersen-64 using the raw bit encoding of the input and returns the digest as `i16`.
fn hash_bool_to_i16_raw(x: bool) -> i16 {
    return _pedersen64_hash_to_i16_raw(x);
}

// Hashes `x` with Pedersen-64 using the raw bit encoding of the input and returns the digest as `i32`.
fn hash_bool_to_i32_raw(x: bool) -> i32 {
    return _pedersen64_hash_to_i32_raw(x);
}

// Hashes `x` with Pedersen-64 using the raw bit encoding of the input and returns the digest as `i64`.
fn hash_bool_to_i64_raw(x: bool) -> i64 {
    return _pedersen64_hash_to_i64_raw(x);
}

// Hashes `x` with Pedersen-64 using the raw bit encoding of the input and returns the digest as `i128`.
fn hash_bool_to_i128_raw(x: bool) -> i128 {
    return _pedersen64_hash_to_i128_raw(x);
}

// Hashes `x` with Pedersen-64 using the raw bit encoding of the input and returns the digest as `address`.
fn hash_u8_to_address_raw(x: u8) -> address {
    return _pedersen64_hash_to_address_raw(x);
}

// Hashes `x` with Pedersen-64 using the raw bit encoding of the input and returns the digest as `field`.
fn hash_u8_to_field_raw(x: u8) -> field {
    return _pedersen64_hash_to_field_raw(x);
}

// Hashes `x` with Pedersen-64 using the raw bit encoding of the input and returns the digest as `group`.
fn hash_u8_to_group_raw(x: u8) -> group {
    return _pedersen64_hash_to_group_raw(x);
}

// Hashes `x` with Pedersen-64 using the raw bit encoding of the input and returns the digest as `scalar`.
fn hash_u8_to_scalar_raw(x: u8) -> scalar {
    return _pedersen64_hash_to_scalar_raw(x);
}

// Hashes `x` with Pedersen-64 using the raw bit encoding of the input and returns the digest as `u8`.
fn hash_u8_to_u8_raw(x: u8) -> u8 {
    return _pedersen64_hash_to_u8_raw(x);
}

// Hashes `x` with Pedersen-64 using the raw bit encoding of the input and returns the digest as `u16`.
fn hash_u8_to_u16_raw(x: u8) -> u16 {
    return _pedersen64_hash_to_u16_raw(x);
}

// Hashes `x` with Pedersen-64 using the raw bit encoding of the input and returns the digest as `u32`.
fn hash_u8_to_u32_raw(x: u8) -> u32 {
    return _pedersen64_hash_to_u32_raw(x);
}

// Hashes `x` with Pedersen-64 using the raw bit encoding of the input and returns the digest as `u64`.
fn hash_u8_to_u64_raw(x: u8) -> u64 {
    return _pedersen64_hash_to_u64_raw(x);
}

// Hashes `x` with Pedersen-64 using the raw bit encoding of the input and returns the digest as `u128`.
fn hash_u8_to_u128_raw(x: u8) -> u128 {
    return _pedersen64_hash_to_u128_raw(x);
}

// Hashes `x` with Pedersen-64 using the raw bit encoding of the input and returns the digest as `i8`.
fn hash_u8_to_i8_raw(x: u8) -> i8 {
    return _pedersen64_hash_to_i8_raw(x);
}

// Hashes `x` with Pedersen-64 using the raw bit encoding of the input and returns the digest as `i16`.
fn hash_u8_to_i16_raw(x: u8) -> i16 {
    return _pedersen64_hash_to_i16_raw(x);
}

// Hashes `x` with Pedersen-64 using the raw bit encoding of the input and returns the digest as `i32`.
fn hash_u8_to_i32_raw(x: u8) -> i32 {
    return _pedersen64_hash_to_i32_raw(x);
}

// Hashes `x` with Pedersen-64 using the raw bit encoding of the input and returns the digest as `i64`.
fn hash_u8_to_i64_raw(x: u8) -> i64 {
    return _pedersen64_hash_to_i64_raw(x);
}

// Hashes `x` with Pedersen-64 using the raw bit encoding of the input and returns the digest as `i128`.
fn hash_u8_to_i128_raw(x: u8) -> i128 {
    return _pedersen64_hash_to_i128_raw(x);
}

// Hashes `x` with Pedersen-64 using the raw bit encoding of the input and returns the digest as `address`.
fn hash_u16_to_address_raw(x: u16) -> address {
    return _pedersen64_hash_to_address_raw(x);
}

// Hashes `x` with Pedersen-64 using the raw bit encoding of the input and returns the digest as `field`.
fn hash_u16_to_field_raw(x: u16) -> field {
    return _pedersen64_hash_to_field_raw(x);
}

// Hashes `x` with Pedersen-64 using the raw bit encoding of the input and returns the digest as `group`.
fn hash_u16_to_group_raw(x: u16) -> group {
    return _pedersen64_hash_to_group_raw(x);
}

// Hashes `x` with Pedersen-64 using the raw bit encoding of the input and returns the digest as `scalar`.
fn hash_u16_to_scalar_raw(x: u16) -> scalar {
    return _pedersen64_hash_to_scalar_raw(x);
}

// Hashes `x` with Pedersen-64 using the raw bit encoding of the input and returns the digest as `u8`.
fn hash_u16_to_u8_raw(x: u16) -> u8 {
    return _pedersen64_hash_to_u8_raw(x);
}

// Hashes `x` with Pedersen-64 using the raw bit encoding of the input and returns the digest as `u16`.
fn hash_u16_to_u16_raw(x: u16) -> u16 {
    return _pedersen64_hash_to_u16_raw(x);
}

// Hashes `x` with Pedersen-64 using the raw bit encoding of the input and returns the digest as `u32`.
fn hash_u16_to_u32_raw(x: u16) -> u32 {
    return _pedersen64_hash_to_u32_raw(x);
}

// Hashes `x` with Pedersen-64 using the raw bit encoding of the input and returns the digest as `u64`.
fn hash_u16_to_u64_raw(x: u16) -> u64 {
    return _pedersen64_hash_to_u64_raw(x);
}

// Hashes `x` with Pedersen-64 using the raw bit encoding of the input and returns the digest as `u128`.
fn hash_u16_to_u128_raw(x: u16) -> u128 {
    return _pedersen64_hash_to_u128_raw(x);
}

// Hashes `x` with Pedersen-64 using the raw bit encoding of the input and returns the digest as `i8`.
fn hash_u16_to_i8_raw(x: u16) -> i8 {
    return _pedersen64_hash_to_i8_raw(x);
}

// Hashes `x` with Pedersen-64 using the raw bit encoding of the input and returns the digest as `i16`.
fn hash_u16_to_i16_raw(x: u16) -> i16 {
    return _pedersen64_hash_to_i16_raw(x);
}

// Hashes `x` with Pedersen-64 using the raw bit encoding of the input and returns the digest as `i32`.
fn hash_u16_to_i32_raw(x: u16) -> i32 {
    return _pedersen64_hash_to_i32_raw(x);
}

// Hashes `x` with Pedersen-64 using the raw bit encoding of the input and returns the digest as `i64`.
fn hash_u16_to_i64_raw(x: u16) -> i64 {
    return _pedersen64_hash_to_i64_raw(x);
}

// Hashes `x` with Pedersen-64 using the raw bit encoding of the input and returns the digest as `i128`.
fn hash_u16_to_i128_raw(x: u16) -> i128 {
    return _pedersen64_hash_to_i128_raw(x);
}

// Hashes `x` with Pedersen-64 using the raw bit encoding of the input and returns the digest as `address`.
fn hash_u32_to_address_raw(x: u32) -> address {
    return _pedersen64_hash_to_address_raw(x);
}

// Hashes `x` with Pedersen-64 using the raw bit encoding of the input and returns the digest as `field`.
fn hash_u32_to_field_raw(x: u32) -> field {
    return _pedersen64_hash_to_field_raw(x);
}

// Hashes `x` with Pedersen-64 using the raw bit encoding of the input and returns the digest as `group`.
fn hash_u32_to_group_raw(x: u32) -> group {
    return _pedersen64_hash_to_group_raw(x);
}

// Hashes `x` with Pedersen-64 using the raw bit encoding of the input and returns the digest as `scalar`.
fn hash_u32_to_scalar_raw(x: u32) -> scalar {
    return _pedersen64_hash_to_scalar_raw(x);
}

// Hashes `x` with Pedersen-64 using the raw bit encoding of the input and returns the digest as `u8`.
fn hash_u32_to_u8_raw(x: u32) -> u8 {
    return _pedersen64_hash_to_u8_raw(x);
}

// Hashes `x` with Pedersen-64 using the raw bit encoding of the input and returns the digest as `u16`.
fn hash_u32_to_u16_raw(x: u32) -> u16 {
    return _pedersen64_hash_to_u16_raw(x);
}

// Hashes `x` with Pedersen-64 using the raw bit encoding of the input and returns the digest as `u32`.
fn hash_u32_to_u32_raw(x: u32) -> u32 {
    return _pedersen64_hash_to_u32_raw(x);
}

// Hashes `x` with Pedersen-64 using the raw bit encoding of the input and returns the digest as `u64`.
fn hash_u32_to_u64_raw(x: u32) -> u64 {
    return _pedersen64_hash_to_u64_raw(x);
}

// Hashes `x` with Pedersen-64 using the raw bit encoding of the input and returns the digest as `u128`.
fn hash_u32_to_u128_raw(x: u32) -> u128 {
    return _pedersen64_hash_to_u128_raw(x);
}

// Hashes `x` with Pedersen-64 using the raw bit encoding of the input and returns the digest as `i8`.
fn hash_u32_to_i8_raw(x: u32) -> i8 {
    return _pedersen64_hash_to_i8_raw(x);
}

// Hashes `x` with Pedersen-64 using the raw bit encoding of the input and returns the digest as `i16`.
fn hash_u32_to_i16_raw(x: u32) -> i16 {
    return _pedersen64_hash_to_i16_raw(x);
}

// Hashes `x` with Pedersen-64 using the raw bit encoding of the input and returns the digest as `i32`.
fn hash_u32_to_i32_raw(x: u32) -> i32 {
    return _pedersen64_hash_to_i32_raw(x);
}

// Hashes `x` with Pedersen-64 using the raw bit encoding of the input and returns the digest as `i64`.
fn hash_u32_to_i64_raw(x: u32) -> i64 {
    return _pedersen64_hash_to_i64_raw(x);
}

// Hashes `x` with Pedersen-64 using the raw bit encoding of the input and returns the digest as `i128`.
fn hash_u32_to_i128_raw(x: u32) -> i128 {
    return _pedersen64_hash_to_i128_raw(x);
}

// Hashes `x` with Pedersen-64 using the raw bit encoding of the input and returns the digest as `address`.
fn hash_i8_to_address_raw(x: i8) -> address {
    return _pedersen64_hash_to_address_raw(x);
}

// Hashes `x` with Pedersen-64 using the raw bit encoding of the input and returns the digest as `field`.
fn hash_i8_to_field_raw(x: i8) -> field {
    return _pedersen64_hash_to_field_raw(x);
}

// Hashes `x` with Pedersen-64 using the raw bit encoding of the input and returns the digest as `group`.
fn hash_i8_to_group_raw(x: i8) -> group {
    return _pedersen64_hash_to_group_raw(x);
}

// Hashes `x` with Pedersen-64 using the raw bit encoding of the input and returns the digest as `scalar`.
fn hash_i8_to_scalar_raw(x: i8) -> scalar {
    return _pedersen64_hash_to_scalar_raw(x);
}

// Hashes `x` with Pedersen-64 using the raw bit encoding of the input and returns the digest as `u8`.
fn hash_i8_to_u8_raw(x: i8) -> u8 {
    return _pedersen64_hash_to_u8_raw(x);
}

// Hashes `x` with Pedersen-64 using the raw bit encoding of the input and returns the digest as `u16`.
fn hash_i8_to_u16_raw(x: i8) -> u16 {
    return _pedersen64_hash_to_u16_raw(x);
}

// Hashes `x` with Pedersen-64 using the raw bit encoding of the input and returns the digest as `u32`.
fn hash_i8_to_u32_raw(x: i8) -> u32 {
    return _pedersen64_hash_to_u32_raw(x);
}

// Hashes `x` with Pedersen-64 using the raw bit encoding of the input and returns the digest as `u64`.
fn hash_i8_to_u64_raw(x: i8) -> u64 {
    return _pedersen64_hash_to_u64_raw(x);
}

// Hashes `x` with Pedersen-64 using the raw bit encoding of the input and returns the digest as `u128`.
fn hash_i8_to_u128_raw(x: i8) -> u128 {
    return _pedersen64_hash_to_u128_raw(x);
}

// Hashes `x` with Pedersen-64 using the raw bit encoding of the input and returns the digest as `i8`.
fn hash_i8_to_i8_raw(x: i8) -> i8 {
    return _pedersen64_hash_to_i8_raw(x);
}

// Hashes `x` with Pedersen-64 using the raw bit encoding of the input and returns the digest as `i16`.
fn hash_i8_to_i16_raw(x: i8) -> i16 {
    return _pedersen64_hash_to_i16_raw(x);
}

// Hashes `x` with Pedersen-64 using the raw bit encoding of the input and returns the digest as `i32`.
fn hash_i8_to_i32_raw(x: i8) -> i32 {
    return _pedersen64_hash_to_i32_raw(x);
}

// Hashes `x` with Pedersen-64 using the raw bit encoding of the input and returns the digest as `i64`.
fn hash_i8_to_i64_raw(x: i8) -> i64 {
    return _pedersen64_hash_to_i64_raw(x);
}

// Hashes `x` with Pedersen-64 using the raw bit encoding of the input and returns the digest as `i128`.
fn hash_i8_to_i128_raw(x: i8) -> i128 {
    return _pedersen64_hash_to_i128_raw(x);
}

// Hashes `x` with Pedersen-64 using the raw bit encoding of the input and returns the digest as `address`.
fn hash_i16_to_address_raw(x: i16) -> address {
    return _pedersen64_hash_to_address_raw(x);
}

// Hashes `x` with Pedersen-64 using the raw bit encoding of the input and returns the digest as `field`.
fn hash_i16_to_field_raw(x: i16) -> field {
    return _pedersen64_hash_to_field_raw(x);
}

// Hashes `x` with Pedersen-64 using the raw bit encoding of the input and returns the digest as `group`.
fn hash_i16_to_group_raw(x: i16) -> group {
    return _pedersen64_hash_to_group_raw(x);
}

// Hashes `x` with Pedersen-64 using the raw bit encoding of the input and returns the digest as `scalar`.
fn hash_i16_to_scalar_raw(x: i16) -> scalar {
    return _pedersen64_hash_to_scalar_raw(x);
}

// Hashes `x` with Pedersen-64 using the raw bit encoding of the input and returns the digest as `u8`.
fn hash_i16_to_u8_raw(x: i16) -> u8 {
    return _pedersen64_hash_to_u8_raw(x);
}

// Hashes `x` with Pedersen-64 using the raw bit encoding of the input and returns the digest as `u16`.
fn hash_i16_to_u16_raw(x: i16) -> u16 {
    return _pedersen64_hash_to_u16_raw(x);
}

// Hashes `x` with Pedersen-64 using the raw bit encoding of the input and returns the digest as `u32`.
fn hash_i16_to_u32_raw(x: i16) -> u32 {
    return _pedersen64_hash_to_u32_raw(x);
}

// Hashes `x` with Pedersen-64 using the raw bit encoding of the input and returns the digest as `u64`.
fn hash_i16_to_u64_raw(x: i16) -> u64 {
    return _pedersen64_hash_to_u64_raw(x);
}

// Hashes `x` with Pedersen-64 using the raw bit encoding of the input and returns the digest as `u128`.
fn hash_i16_to_u128_raw(x: i16) -> u128 {
    return _pedersen64_hash_to_u128_raw(x);
}

// Hashes `x` with Pedersen-64 using the raw bit encoding of the input and returns the digest as `i8`.
fn hash_i16_to_i8_raw(x: i16) -> i8 {
    return _pedersen64_hash_to_i8_raw(x);
}

// Hashes `x` with Pedersen-64 using the raw bit encoding of the input and returns the digest as `i16`.
fn hash_i16_to_i16_raw(x: i16) -> i16 {
    return _pedersen64_hash_to_i16_raw(x);
}

// Hashes `x` with Pedersen-64 using the raw bit encoding of the input and returns the digest as `i32`.
fn hash_i16_to_i32_raw(x: i16) -> i32 {
    return _pedersen64_hash_to_i32_raw(x);
}

// Hashes `x` with Pedersen-64 using the raw bit encoding of the input and returns the digest as `i64`.
fn hash_i16_to_i64_raw(x: i16) -> i64 {
    return _pedersen64_hash_to_i64_raw(x);
}

// Hashes `x` with Pedersen-64 using the raw bit encoding of the input and returns the digest as `i128`.
fn hash_i16_to_i128_raw(x: i16) -> i128 {
    return _pedersen64_hash_to_i128_raw(x);
}

// Hashes `x` with Pedersen-64 using the raw bit encoding of the input and returns the digest as `address`.
fn hash_i32_to_address_raw(x: i32) -> address {
    return _pedersen64_hash_to_address_raw(x);
}

// Hashes `x` with Pedersen-64 using the raw bit encoding of the input and returns the digest as `field`.
fn hash_i32_to_field_raw(x: i32) -> field {
    return _pedersen64_hash_to_field_raw(x);
}

// Hashes `x` with Pedersen-64 using the raw bit encoding of the input and returns the digest as `group`.
fn hash_i32_to_group_raw(x: i32) -> group {
    return _pedersen64_hash_to_group_raw(x);
}

// Hashes `x` with Pedersen-64 using the raw bit encoding of the input and returns the digest as `scalar`.
fn hash_i32_to_scalar_raw(x: i32) -> scalar {
    return _pedersen64_hash_to_scalar_raw(x);
}

// Hashes `x` with Pedersen-64 using the raw bit encoding of the input and returns the digest as `u8`.
fn hash_i32_to_u8_raw(x: i32) -> u8 {
    return _pedersen64_hash_to_u8_raw(x);
}

// Hashes `x` with Pedersen-64 using the raw bit encoding of the input and returns the digest as `u16`.
fn hash_i32_to_u16_raw(x: i32) -> u16 {
    return _pedersen64_hash_to_u16_raw(x);
}

// Hashes `x` with Pedersen-64 using the raw bit encoding of the input and returns the digest as `u32`.
fn hash_i32_to_u32_raw(x: i32) -> u32 {
    return _pedersen64_hash_to_u32_raw(x);
}

// Hashes `x` with Pedersen-64 using the raw bit encoding of the input and returns the digest as `u64`.
fn hash_i32_to_u64_raw(x: i32) -> u64 {
    return _pedersen64_hash_to_u64_raw(x);
}

// Hashes `x` with Pedersen-64 using the raw bit encoding of the input and returns the digest as `u128`.
fn hash_i32_to_u128_raw(x: i32) -> u128 {
    return _pedersen64_hash_to_u128_raw(x);
}

// Hashes `x` with Pedersen-64 using the raw bit encoding of the input and returns the digest as `i8`.
fn hash_i32_to_i8_raw(x: i32) -> i8 {
    return _pedersen64_hash_to_i8_raw(x);
}

// Hashes `x` with Pedersen-64 using the raw bit encoding of the input and returns the digest as `i16`.
fn hash_i32_to_i16_raw(x: i32) -> i16 {
    return _pedersen64_hash_to_i16_raw(x);
}

// Hashes `x` with Pedersen-64 using the raw bit encoding of the input and returns the digest as `i32`.
fn hash_i32_to_i32_raw(x: i32) -> i32 {
    return _pedersen64_hash_to_i32_raw(x);
}

// Hashes `x` with Pedersen-64 using the raw bit encoding of the input and returns the digest as `i64`.
fn hash_i32_to_i64_raw(x: i32) -> i64 {
    return _pedersen64_hash_to_i64_raw(x);
}

// Hashes `x` with Pedersen-64 using the raw bit encoding of the input and returns the digest as `i128`.
fn hash_i32_to_i128_raw(x: i32) -> i128 {
    return _pedersen64_hash_to_i128_raw(x);
}

