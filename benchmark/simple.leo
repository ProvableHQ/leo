function main(a: private bool) -> (u32) {
  b = 0;
  for i in 0..4 {
    if (a) {
        return 9
    } else {
      b = b + i;
    }
  }
  return b
}