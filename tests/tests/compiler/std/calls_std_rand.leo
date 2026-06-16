program main.aleo {
    fn exercise_async() -> Final {
        return final {
            let rf1 = std::rand::chacha_address();
            assert_eq(rf1, rf1);
            let rf2 = std::rand::chacha_bool();
            assert_eq(rf2, rf2);
            let rf3 = std::rand::chacha_field();
            assert_eq(rf3, rf3);
            let rf4 = std::rand::chacha_group();
            assert_eq(rf4, rf4);
            let rf5 = std::rand::chacha_scalar();
            assert_eq(rf5, rf5);
            let rf6 = std::rand::chacha_u8();
            assert_eq(rf6, rf6);
            let rf7 = std::rand::chacha_u16();
            assert_eq(rf7, rf7);
            let rf8 = std::rand::chacha_u32();
            assert_eq(rf8, rf8);
            let rf9 = std::rand::chacha_u64();
            assert_eq(rf9, rf9);
            let rf10 = std::rand::chacha_u128();
            assert_eq(rf10, rf10);
            let rf11 = std::rand::chacha_i8();
            assert_eq(rf11, rf11);
            let rf12 = std::rand::chacha_i16();
            assert_eq(rf12, rf12);
            let rf13 = std::rand::chacha_i32();
            assert_eq(rf13, rf13);
            let rf14 = std::rand::chacha_i64();
            assert_eq(rf14, rf14);
            let rf15 = std::rand::chacha_i128();
            assert_eq(rf15, rf15);
        };
    }

    @noupgrade
    constructor() {}
}
