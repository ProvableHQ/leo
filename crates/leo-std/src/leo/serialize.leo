// Bit-level serialization and deserialization of primitive types.
//
// Every Leo primitive has a canonical bit representation that this module
// makes available as a fixed-size `[bool; N]` array. The encoding round-trips:
// `from_bits(to_bits(x)) == x` for any value `x` of a wrapped type, and
// likewise for the `_raw` variants.
//
// # Tagged vs. raw encoding
//
// Two encodings are exposed for each primitive:
//
// - `to_bits` / `from_bits` prepend a 26-bit **type discriminator** to the
//   value's native bit representation. The discriminator identifies the
//   source type, so a deserializer can reject a bit string that was produced
//   for a different type. Use the tagged form whenever the bits will be
//   stored, hashed into a commitment, or transmitted between programs, any
//   context where type confusion would be a security issue.
//
// - `to_bits_raw` / `from_bits_raw` use the native bit width of the type
//   with no tag. The output array is shorter, but two values of different
//   types may share the same bit pattern (e.g. `8u8` and `8i8`). Use the
//   raw form inside a single algorithm where the types are fixed and known
//   on both sides.
//
// # Bit widths
//
// Native widths: `bool = 1`, `uN/iN = N`, `field/group/address = 253`,
// `scalar = 251`. The tagged form adds 26 bits to each width, for example,
// `to_bits_field` returns `[bool; 279]` and `to_bits_raw_field` returns
// `[bool; 253]`.

// Tagged encoding of a `bool`, `[bool; 27]` (1 native bit + 26-bit type tag).
fn to_bits_bool(x: bool) -> [bool; 27] {
    return _serialize_to_bits(x);
}

// Tagged encoding of a `u8`, `[bool; 34]`.
fn to_bits_u8(x: u8) -> [bool; 34] {
    return _serialize_to_bits(x);
}

// Tagged encoding of a `u16`, `[bool; 42]`.
fn to_bits_u16(x: u16) -> [bool; 42] {
    return _serialize_to_bits(x);
}

// Tagged encoding of a `u32`, `[bool; 58]`.
fn to_bits_u32(x: u32) -> [bool; 58] {
    return _serialize_to_bits(x);
}

// Tagged encoding of a `u64`, `[bool; 90]`.
fn to_bits_u64(x: u64) -> [bool; 90] {
    return _serialize_to_bits(x);
}

// Tagged encoding of a `u128`, `[bool; 154]`.
fn to_bits_u128(x: u128) -> [bool; 154] {
    return _serialize_to_bits(x);
}

// Tagged encoding of an `i8`, `[bool; 34]`.
fn to_bits_i8(x: i8) -> [bool; 34] {
    return _serialize_to_bits(x);
}

// Tagged encoding of an `i16`, `[bool; 42]`.
fn to_bits_i16(x: i16) -> [bool; 42] {
    return _serialize_to_bits(x);
}

// Tagged encoding of an `i32`, `[bool; 58]`.
fn to_bits_i32(x: i32) -> [bool; 58] {
    return _serialize_to_bits(x);
}

// Tagged encoding of an `i64`, `[bool; 90]`.
fn to_bits_i64(x: i64) -> [bool; 90] {
    return _serialize_to_bits(x);
}

// Tagged encoding of an `i128`, `[bool; 154]`.
fn to_bits_i128(x: i128) -> [bool; 154] {
    return _serialize_to_bits(x);
}

// Tagged encoding of a `field`, `[bool; 279]`.
fn to_bits_field(x: field) -> [bool; 279] {
    return _serialize_to_bits(x);
}

// Tagged encoding of a `group`, `[bool; 279]`.
fn to_bits_group(x: group) -> [bool; 279] {
    return _serialize_to_bits(x);
}

// Tagged encoding of a `scalar`, `[bool; 277]`.
fn to_bits_scalar(x: scalar) -> [bool; 277] {
    return _serialize_to_bits(x);
}

// Tagged encoding of an `address`, `[bool; 279]`.
fn to_bits_address(x: address) -> [bool; 279] {
    return _serialize_to_bits(x);
}

// Raw encoding of a `bool`, `[bool; 1]`, just the value's native bit.
fn to_bits_raw_bool(x: bool) -> [bool; 1] {
    return _serialize_to_bits_raw(x);
}

// Raw encoding of a `u8`, `[bool; 8]`.
fn to_bits_raw_u8(x: u8) -> [bool; 8] {
    return _serialize_to_bits_raw(x);
}

// Raw encoding of a `u16`, `[bool; 16]`.
fn to_bits_raw_u16(x: u16) -> [bool; 16] {
    return _serialize_to_bits_raw(x);
}

// Raw encoding of a `u32`, `[bool; 32]`.
fn to_bits_raw_u32(x: u32) -> [bool; 32] {
    return _serialize_to_bits_raw(x);
}

// Raw encoding of a `u64`, `[bool; 64]`.
fn to_bits_raw_u64(x: u64) -> [bool; 64] {
    return _serialize_to_bits_raw(x);
}

// Raw encoding of a `u128`, `[bool; 128]`.
fn to_bits_raw_u128(x: u128) -> [bool; 128] {
    return _serialize_to_bits_raw(x);
}

// Raw encoding of an `i8`, `[bool; 8]`.
fn to_bits_raw_i8(x: i8) -> [bool; 8] {
    return _serialize_to_bits_raw(x);
}

// Raw encoding of an `i16`, `[bool; 16]`.
fn to_bits_raw_i16(x: i16) -> [bool; 16] {
    return _serialize_to_bits_raw(x);
}

// Raw encoding of an `i32`, `[bool; 32]`.
fn to_bits_raw_i32(x: i32) -> [bool; 32] {
    return _serialize_to_bits_raw(x);
}

// Raw encoding of an `i64`, `[bool; 64]`.
fn to_bits_raw_i64(x: i64) -> [bool; 64] {
    return _serialize_to_bits_raw(x);
}

// Raw encoding of an `i128`, `[bool; 128]`.
fn to_bits_raw_i128(x: i128) -> [bool; 128] {
    return _serialize_to_bits_raw(x);
}

// Raw encoding of a `field`, `[bool; 253]`.
fn to_bits_raw_field(x: field) -> [bool; 253] {
    return _serialize_to_bits_raw(x);
}

// Raw encoding of a `group`, `[bool; 253]`.
fn to_bits_raw_group(x: group) -> [bool; 253] {
    return _serialize_to_bits_raw(x);
}

// Raw encoding of a `scalar`, `[bool; 251]`.
fn to_bits_raw_scalar(x: scalar) -> [bool; 251] {
    return _serialize_to_bits_raw(x);
}

// Raw encoding of an `address`, `[bool; 253]`.
fn to_bits_raw_address(x: address) -> [bool; 253] {
    return _serialize_to_bits_raw(x);
}

// Decodes the tagged bit encoding of a `field`. Fails if `bits` does not
// carry the field discriminator or is not a valid encoding.
fn from_bits_field(bits: [bool; 279]) -> field {
    return _deserialize_from_bits::[field](bits);
}

// Decodes the tagged bit encoding of a `scalar`. Fails if the discriminator
// does not match the scalar type.
fn from_bits_scalar(bits: [bool; 277]) -> scalar {
    return _deserialize_from_bits::[scalar](bits);
}

// Decodes the tagged bit encoding of a `group`. Fails if the discriminator
// does not match or the value is not on the curve.
fn from_bits_group(bits: [bool; 279]) -> group {
    return _deserialize_from_bits::[group](bits);
}

// Decodes the tagged bit encoding of an `address`. Fails if the discriminator
// does not match the address type.
fn from_bits_address(bits: [bool; 279]) -> address {
    return _deserialize_from_bits::[address](bits);
}

// Decodes the tagged bit encoding of a `u8`.
fn from_bits_u8(bits: [bool; 34]) -> u8 {
    return _deserialize_from_bits::[u8](bits);
}

// Decodes the tagged bit encoding of a `u16`.
fn from_bits_u16(bits: [bool; 42]) -> u16 {
    return _deserialize_from_bits::[u16](bits);
}

// Decodes the tagged bit encoding of a `u32`.
fn from_bits_u32(bits: [bool; 58]) -> u32 {
    return _deserialize_from_bits::[u32](bits);
}

// Decodes the tagged bit encoding of a `u64`.
fn from_bits_u64(bits: [bool; 90]) -> u64 {
    return _deserialize_from_bits::[u64](bits);
}

// Decodes the tagged bit encoding of a `u128`.
fn from_bits_u128(bits: [bool; 154]) -> u128 {
    return _deserialize_from_bits::[u128](bits);
}

// Decodes the tagged bit encoding of an `i8`.
fn from_bits_i8(bits: [bool; 34]) -> i8 {
    return _deserialize_from_bits::[i8](bits);
}

// Decodes the tagged bit encoding of an `i16`.
fn from_bits_i16(bits: [bool; 42]) -> i16 {
    return _deserialize_from_bits::[i16](bits);
}

// Decodes the tagged bit encoding of an `i32`.
fn from_bits_i32(bits: [bool; 58]) -> i32 {
    return _deserialize_from_bits::[i32](bits);
}

// Decodes the tagged bit encoding of an `i64`.
fn from_bits_i64(bits: [bool; 90]) -> i64 {
    return _deserialize_from_bits::[i64](bits);
}

// Decodes the tagged bit encoding of an `i128`.
fn from_bits_i128(bits: [bool; 154]) -> i128 {
    return _deserialize_from_bits::[i128](bits);
}

// Decodes the raw bit encoding of a `field`. The caller must know the bits
// came from a `field` value; no tag is checked.
fn from_bits_raw_field(bits: [bool; 253]) -> field {
    return _deserialize_from_bits_raw::[field](bits);
}

// Decodes the raw bit encoding of a `scalar`.
fn from_bits_raw_scalar(bits: [bool; 251]) -> scalar {
    return _deserialize_from_bits_raw::[scalar](bits);
}

// Decodes the raw bit encoding of a `group`. Fails if the bits do not encode
// a point on the curve.
fn from_bits_raw_group(bits: [bool; 253]) -> group {
    return _deserialize_from_bits_raw::[group](bits);
}

// Decodes the raw bit encoding of an `address`.
fn from_bits_raw_address(bits: [bool; 253]) -> address {
    return _deserialize_from_bits_raw::[address](bits);
}

// Decodes the raw bit encoding of a `u8`.
fn from_bits_raw_u8(bits: [bool; 8]) -> u8 {
    return _deserialize_from_bits_raw::[u8](bits);
}

// Decodes the raw bit encoding of a `u16`.
fn from_bits_raw_u16(bits: [bool; 16]) -> u16 {
    return _deserialize_from_bits_raw::[u16](bits);
}

// Decodes the raw bit encoding of a `u32`.
fn from_bits_raw_u32(bits: [bool; 32]) -> u32 {
    return _deserialize_from_bits_raw::[u32](bits);
}

// Decodes the raw bit encoding of a `u64`.
fn from_bits_raw_u64(bits: [bool; 64]) -> u64 {
    return _deserialize_from_bits_raw::[u64](bits);
}

// Decodes the raw bit encoding of a `u128`.
fn from_bits_raw_u128(bits: [bool; 128]) -> u128 {
    return _deserialize_from_bits_raw::[u128](bits);
}

// Decodes the raw bit encoding of an `i8`.
fn from_bits_raw_i8(bits: [bool; 8]) -> i8 {
    return _deserialize_from_bits_raw::[i8](bits);
}

// Decodes the raw bit encoding of an `i16`.
fn from_bits_raw_i16(bits: [bool; 16]) -> i16 {
    return _deserialize_from_bits_raw::[i16](bits);
}

// Decodes the raw bit encoding of an `i32`.
fn from_bits_raw_i32(bits: [bool; 32]) -> i32 {
    return _deserialize_from_bits_raw::[i32](bits);
}

// Decodes the raw bit encoding of an `i64`.
fn from_bits_raw_i64(bits: [bool; 64]) -> i64 {
    return _deserialize_from_bits_raw::[i64](bits);
}

// Decodes the raw bit encoding of an `i128`.
fn from_bits_raw_i128(bits: [bool; 128]) -> i128 {
    return _deserialize_from_bits_raw::[i128](bits);
}
