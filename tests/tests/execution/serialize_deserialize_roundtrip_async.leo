/*
seed = 123456789
min_height = 16

[case]
program = "test.aleo"
function = "test_async_u32_rt"
input = ["99999u32"]
private_key = "APrivateKey1zkpH5Ne1Xfd79t61VhK7b6yaYz92yW5dbuVkiFheR7rwCDE"
[case]
program = "test.aleo"
function = "test_async_field_rt"
input = ["999999field"]
private_key = "APrivateKey1zkpH5Ne1Xfd79t61VhK7b6yaYz92yW5dbuVkiFheR7rwCDE"
[case]
program = "test.aleo"
function = "test_async_array_rt"
input = ["[100u8, 200u8]"]
private_key = "APrivateKey1zkpH5Ne1Xfd79t61VhK7b6yaYz92yW5dbuVkiFheR7rwCDE"
*/

final fn fin_test_async_u32_rt(addr: address, value: u32) {
    let bits: [bool; 58] = Serialize::to_bits(value);
    let reconstructed: u32 = Deserialize::from_bits::[u32](bits);
    let matches_non_raw: bool = value == reconstructed;

    let bits_raw: [bool; 32] = Serialize::to_bits_raw(value);
    let reconstructed_raw: u32 = Deserialize::from_bits_raw::[u32](bits_raw);
    let matches_raw: bool = value == reconstructed_raw;

    assert(matches_non_raw && matches_raw);
}

final fn fin_test_async_field_rt(addr: address, value: field) {
    let bits: [bool; 279] = Serialize::to_bits(value);
    let reconstructed: field = Deserialize::from_bits::[field](bits);
    let matches_non_raw: bool = value == reconstructed;

    let bits_raw: [bool; 253] = Serialize::to_bits_raw(value);
    let reconstructed_raw: field = Deserialize::from_bits_raw::[field](bits_raw);
    let matches_raw: bool = value == reconstructed_raw;

    assert(matches_non_raw && matches_raw);
}

final fn fin_test_async_array_rt(addr: address, value: [u8; 2]) {
    let bits: [bool; 134] = Serialize::to_bits(value);
    let reconstructed: [u8; 2] = Deserialize::from_bits::[[u8; 2]](bits);
    let matches_non_raw: bool =
        value[0u8] == reconstructed[0u8] &&
        value[1u8] == reconstructed[1u8];

    let bits_raw: [bool; 16] = Serialize::to_bits_raw(value);
    let reconstructed_raw: [u8; 2] = Deserialize::from_bits_raw::[[u8; 2]](bits_raw);
    let matches_raw: bool =
        value[0u8] == reconstructed_raw[0u8] &&
        value[1u8] == reconstructed_raw[1u8];

    assert(matches_non_raw && matches_raw);
}

program test.aleo {
    fn test_async_u32_rt(value: u32) -> Final {
        let caller = self.caller;
        return final { fin_test_async_u32_rt(caller, value); };
    }

    fn test_async_field_rt(value: field) -> Final {
        let caller = self.caller;
        return final { fin_test_async_field_rt(caller, value); };
    }

    fn test_async_array_rt(value: [u8; 2]) -> Final {
        let caller = self.caller;
        return final { fin_test_async_array_rt(caller, value); };
    }

    @noupgrade
    constructor() {}
}
