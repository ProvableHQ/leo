program main.aleo {
    fn exercise() {
        let rp1 = std::commit::pedersen64::commit_bool_to_address(true, 1scalar);
        assert_eq(rp1, rp1);
        let rp2 = std::commit::pedersen64::commit_bool_to_field(true, 1scalar);
        assert_eq(rp2, rp2);
        let rp3 = std::commit::pedersen64::commit_bool_to_group(true, 1scalar);
        assert_eq(rp3, rp3);
        let rp4 = std::commit::pedersen64::commit_u8_to_address(1u8, 1scalar);
        assert_eq(rp4, rp4);
        let rp5 = std::commit::pedersen64::commit_u8_to_field(1u8, 1scalar);
        assert_eq(rp5, rp5);
        let rp6 = std::commit::pedersen64::commit_u8_to_group(1u8, 1scalar);
        assert_eq(rp6, rp6);
        let rp7 = std::commit::pedersen64::commit_u16_to_address(1u16, 1scalar);
        assert_eq(rp7, rp7);
        let rp8 = std::commit::pedersen64::commit_u16_to_field(1u16, 1scalar);
        assert_eq(rp8, rp8);
        let rp9 = std::commit::pedersen64::commit_u16_to_group(1u16, 1scalar);
        assert_eq(rp9, rp9);
        let rp10 = std::commit::pedersen64::commit_u32_to_address(1u32, 1scalar);
        assert_eq(rp10, rp10);
        let rp11 = std::commit::pedersen64::commit_u32_to_field(1u32, 1scalar);
        assert_eq(rp11, rp11);
        let rp12 = std::commit::pedersen64::commit_u32_to_group(1u32, 1scalar);
        assert_eq(rp12, rp12);
        let rp13 = std::commit::pedersen64::commit_i8_to_address(1i8, 1scalar);
        assert_eq(rp13, rp13);
        let rp14 = std::commit::pedersen64::commit_i8_to_field(1i8, 1scalar);
        assert_eq(rp14, rp14);
        let rp15 = std::commit::pedersen64::commit_i8_to_group(1i8, 1scalar);
        assert_eq(rp15, rp15);
        let rp16 = std::commit::pedersen64::commit_i16_to_address(1i16, 1scalar);
        assert_eq(rp16, rp16);
        let rp17 = std::commit::pedersen64::commit_i16_to_field(1i16, 1scalar);
        assert_eq(rp17, rp17);
        let rp18 = std::commit::pedersen64::commit_i16_to_group(1i16, 1scalar);
        assert_eq(rp18, rp18);
        let rp19 = std::commit::pedersen64::commit_i32_to_address(1i32, 1scalar);
        assert_eq(rp19, rp19);
        let rp20 = std::commit::pedersen64::commit_i32_to_field(1i32, 1scalar);
        assert_eq(rp20, rp20);
        let rp21 = std::commit::pedersen64::commit_i32_to_group(1i32, 1scalar);
        assert_eq(rp21, rp21);
        return;
    }

    @noupgrade
    constructor() {}
}
