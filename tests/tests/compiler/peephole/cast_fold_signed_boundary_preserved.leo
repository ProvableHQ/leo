program test.aleo {
    fn main(x: u8) -> u8 {
        let signed: i8 = x as i8;
        let unsigned: u8 = signed as u8;
        return unsigned;
    }

    fn signed_to_signed(x: i32) -> i8 {
        let signed: i16 = x as i16;
        let narrower: i8 = signed as i8;
        return narrower;
    }

    fn signed_to_unsigned(x: i32) -> u8 {
        let signed: i16 = x as i16;
        let unsigned: u8 = signed as u8;
        return unsigned;
    }

    fn unsigned_to_signed(x: i16) -> i8 {
        let unsigned: u16 = x as u16;
        let signed: i8 = unsigned as i8;
        return signed;
    }
}
