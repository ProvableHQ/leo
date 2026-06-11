program test.aleo {
    fn main(x: u8) -> u8 {
        let signed: i8 = x as i8;
        let unsigned: u8 = signed as u8;
        return unsigned;
    }
}
