function main(bit: private u32) {
    if bit == 1u32 {
      assert_eq!(bit, 1u32);
    } else {
      assert_eq!(bit, 0u32);
    }
}
