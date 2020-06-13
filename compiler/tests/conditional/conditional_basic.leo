function main(bit: private u8) {
    if bit == 1u8 {
      assert_eq!(bit, 1u8);
    } else {
      assert_eq!(bit, 0u8);
    }
}
