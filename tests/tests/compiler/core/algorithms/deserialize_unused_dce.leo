program test.aleo {
    fn unused_deserialize(bits: [bool; 253]) -> u8 {
        let value: field = Deserialize::from_bits_raw::[field](bits);
        return 1u8;
    }

    fn used_deserialize(bits: [bool; 253]) -> field {
        let value: field = Deserialize::from_bits_raw::[field](bits);
        return value;
    }
}
