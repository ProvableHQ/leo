function main(bit: private u32) {
    if bit == 1 {
      assert_eq!(bit, 1);
    } else {
      assert_eq!(bit, 0);
    }
}
