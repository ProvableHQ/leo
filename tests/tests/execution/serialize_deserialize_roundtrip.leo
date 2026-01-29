/*
seed = 123456789
min_height = 16

[case]
program = "test.aleo"
function = "test_u8_rt"
input = ["42u8"]
[case]
program = "test.aleo"
function = "test_u16_rt"
input = ["1234u16"]
[case]
program = "test.aleo"
function = "test_u32_rt"
input = ["123456u32"]
[case]
program = "test.aleo"
function = "test_u64_rt"
input = ["9876543210u64"]
[case]
program = "test.aleo"
function = "test_u128_rt"
input = ["123456789012345678u128"]
[case]
program = "test.aleo"
function = "test_i8_rt"
input = ["-42i8"]
[case]
program = "test.aleo"
function = "test_i16_rt"
input = ["-1234i16"]
[case]
program = "test.aleo"
function = "test_i32_rt"
input = ["-123456i32"]
[case]
program = "test.aleo"
function = "test_i64_rt"
input = ["-9876543210i64"]
[case]
program = "test.aleo"
function = "test_i128_rt"
input = ["-123456789012345678i128"]
[case]
program = "test.aleo"
function = "test_field_rt"
input = ["123456field"]
[case]
program = "test.aleo"
function = "test_group_rt"
input = ["0group"]
[case]
program = "test.aleo"
function = "test_scalar_rt"
input = ["1scalar"]
[case]
program = "test.aleo"
function = "test_address_rt"
input = ["aleo1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq3ljyzc"]
[case]
program = "test.aleo"
function = "test_bool_rt"
input = ["true"]
[case]
program = "test.aleo"
function = "test_bool_rt"
input = ["false"]
[case]
program = "test.aleo"
function = "test_array_rt"
input = ["[10u32, 20u32, 30u32, 40u32]"]
*/

program test.aleo {
    mapping results: u8 => bool;

    fn test_u8_rt(value: u8) -> public bool {
        let bits: [bool; 34] = Serialize::to_bits(value);
        let reconstructed: u8 = Deserialize::from_bits::[u8](bits);
        let matches_non_raw: bool = value == reconstructed;

        let bits_raw: [bool; 8] = Serialize::to_bits_raw(value);
        let reconstructed_raw: u8 = Deserialize::from_bits_raw::[u8](bits_raw);
        let matches_raw: bool = value == reconstructed_raw;

        return matches_non_raw && matches_raw;
    }

    fn test_u16_rt(value: u16) -> public bool {
        let bits: [bool; 42] = Serialize::to_bits(value);
        let reconstructed: u16 = Deserialize::from_bits::[u16](bits);
        let matches_non_raw: bool = value == reconstructed;

        let bits_raw: [bool; 16] = Serialize::to_bits_raw(value);
        let reconstructed_raw: u16 = Deserialize::from_bits_raw::[u16](bits_raw);
        let matches_raw: bool = value == reconstructed_raw;

        return matches_non_raw && matches_raw;
    }

    fn test_u32_rt(value: u32) -> public bool {
        let bits: [bool; 58] = Serialize::to_bits(value);
        let reconstructed: u32 = Deserialize::from_bits::[u32](bits);
        let matches_non_raw: bool = value == reconstructed;

        let bits_raw: [bool; 32] = Serialize::to_bits_raw(value);
        let reconstructed_raw: u32 = Deserialize::from_bits_raw::[u32](bits_raw);
        let matches_raw: bool = value == reconstructed_raw;

        return matches_non_raw && matches_raw;
    }

    fn test_u64_rt(value: u64) -> public bool {
        let bits: [bool; 90] = Serialize::to_bits(value);
        let reconstructed: u64 = Deserialize::from_bits::[u64](bits);
        let matches_non_raw: bool = value == reconstructed;

        let bits_raw: [bool; 64] = Serialize::to_bits_raw(value);
        let reconstructed_raw: u64 = Deserialize::from_bits_raw::[u64](bits_raw);
        let matches_raw: bool = value == reconstructed_raw;

        return matches_non_raw && matches_raw;
    }

    fn test_u128_rt(value: u128) -> public bool {
        let bits: [bool; 154] = Serialize::to_bits(value);
        let reconstructed: u128 = Deserialize::from_bits::[u128](bits);
        let matches_non_raw: bool = value == reconstructed;

        let bits_raw: [bool; 128] = Serialize::to_bits_raw(value);
        let reconstructed_raw: u128 = Deserialize::from_bits_raw::[u128](bits_raw);
        let matches_raw: bool = value == reconstructed_raw;

        return matches_non_raw && matches_raw;
    }

    fn test_i8_rt(value: i8) -> public bool {
        let bits: [bool; 34] = Serialize::to_bits(value);
        let reconstructed: i8 = Deserialize::from_bits::[i8](bits);
        let matches_non_raw: bool = value == reconstructed;

        let bits_raw: [bool; 8] = Serialize::to_bits_raw(value);
        let reconstructed_raw: i8 = Deserialize::from_bits_raw::[i8](bits_raw);
        let matches_raw: bool = value == reconstructed_raw;

        return matches_non_raw && matches_raw;
    }

    fn test_i16_rt(value: i16) -> public bool {
        let bits: [bool; 42] = Serialize::to_bits(value);
        let reconstructed: i16 = Deserialize::from_bits::[i16](bits);
        let matches_non_raw: bool = value == reconstructed;

        let bits_raw: [bool; 16] = Serialize::to_bits_raw(value);
        let reconstructed_raw: i16 = Deserialize::from_bits_raw::[i16](bits_raw);
        let matches_raw: bool = value == reconstructed_raw;

        return matches_non_raw && matches_raw;
    }

    fn test_i32_rt(value: i32) -> public bool {
        let bits: [bool; 58] = Serialize::to_bits(value);
        let reconstructed: i32 = Deserialize::from_bits::[i32](bits);
        let matches_non_raw: bool = value == reconstructed;

        let bits_raw: [bool; 32] = Serialize::to_bits_raw(value);
        let reconstructed_raw: i32 = Deserialize::from_bits_raw::[i32](bits_raw);
        let matches_raw: bool = value == reconstructed_raw;

        return matches_non_raw && matches_raw;
    }

    fn test_i64_rt(value: i64) -> public bool {
        let bits: [bool; 90] = Serialize::to_bits(value);
        let reconstructed: i64 = Deserialize::from_bits::[i64](bits);
        let matches_non_raw: bool = value == reconstructed;

        let bits_raw: [bool; 64] = Serialize::to_bits_raw(value);
        let reconstructed_raw: i64 = Deserialize::from_bits_raw::[i64](bits_raw);
        let matches_raw: bool = value == reconstructed_raw;

        return matches_non_raw && matches_raw;
    }

    fn test_i128_rt(value: i128) -> public bool {
        let bits: [bool; 154] = Serialize::to_bits(value);
        let reconstructed: i128 = Deserialize::from_bits::[i128](bits);
        let matches_non_raw: bool = value == reconstructed;

        let bits_raw: [bool; 128] = Serialize::to_bits_raw(value);
        let reconstructed_raw: i128 = Deserialize::from_bits_raw::[i128](bits_raw);
        let matches_raw: bool = value == reconstructed_raw;

        return matches_non_raw && matches_raw;
    }

    fn test_field_rt(value: field) -> public bool {
        let bits: [bool; 279] = Serialize::to_bits(value);
        let reconstructed: field = Deserialize::from_bits::[field](bits);
        let matches_non_raw: bool = value == reconstructed;

        let bits_raw: [bool; 253] = Serialize::to_bits_raw(value);
        let reconstructed_raw: field = Deserialize::from_bits_raw::[field](bits_raw);
        let matches_raw: bool = value == reconstructed_raw;

        return matches_non_raw && matches_raw;
    }

    fn test_group_rt(value: group) -> public bool {
        let bits: [bool; 279] = Serialize::to_bits(value);
        let reconstructed: group = Deserialize::from_bits::[group](bits);
        let matches_non_raw: bool = value == reconstructed;

        let bits_raw: [bool; 253] = Serialize::to_bits_raw(value);
        let reconstructed_raw: group = Deserialize::from_bits_raw::[group](bits_raw);
        let matches_raw: bool = value == reconstructed_raw;

        return matches_non_raw && matches_raw;
    }

    fn test_scalar_rt(value: scalar) -> public bool {
        let bits: [bool; 277] = Serialize::to_bits(value);
        let reconstructed: scalar = Deserialize::from_bits::[scalar](bits);
        let matches_non_raw: bool = value == reconstructed;

        let bits_raw: [bool; 251] = Serialize::to_bits_raw(value);
        let reconstructed_raw: scalar = Deserialize::from_bits_raw::[scalar](bits_raw);
        let matches_raw: bool = value == reconstructed_raw;

        return matches_non_raw && matches_raw;
    }

    fn test_address_rt(value: address) -> public bool {
        let bits: [bool; 279] = Serialize::to_bits(value);
        let reconstructed: address = Deserialize::from_bits::[address](bits);
        let matches_non_raw: bool = value == reconstructed;

        let bits_raw: [bool; 253] = Serialize::to_bits_raw(value);
        let reconstructed_raw: address = Deserialize::from_bits_raw::[address](bits_raw);
        let matches_raw: bool = value == reconstructed_raw;

        return matches_non_raw && matches_raw;
    }

    fn test_bool_rt(value: bool) -> public bool {
        let bits: [bool; 27] = Serialize::to_bits(value);
        let reconstructed: bool = Deserialize::from_bits::[bool](bits);
        let matches_non_raw: bool = value == reconstructed;

        let bits_raw: [bool; 1] = Serialize::to_bits_raw(value);
        let reconstructed_raw: bool = Deserialize::from_bits_raw::[bool](bits_raw);
        let matches_raw: bool = value == reconstructed_raw;

        return matches_non_raw && matches_raw;
    }

    fn test_array_rt(value: [u32; 4]) -> public bool {
        let bits: [bool; 330] = Serialize::to_bits(value);
        let reconstructed: [u32; 4] = Deserialize::from_bits::[[u32; 4]](bits);
        let matches_non_raw: bool =
            value[0u8] == reconstructed[0u8] &&
            value[1u8] == reconstructed[1u8] &&
            value[2u8] == reconstructed[2u8] &&
            value[3u8] == reconstructed[3u8];

        let bits_raw: [bool; 128] = Serialize::to_bits_raw(value);
        let reconstructed_raw: [u32; 4] = Deserialize::from_bits_raw::[[u32; 4]](bits_raw);
        let matches_raw: bool =
            value[0u8] == reconstructed_raw[0u8] &&
            value[1u8] == reconstructed_raw[1u8] &&
            value[2u8] == reconstructed_raw[2u8] &&
            value[3u8] == reconstructed_raw[3u8];

        return matches_non_raw && matches_raw;
    }

    @noupgrade
    constructor() {}
}
