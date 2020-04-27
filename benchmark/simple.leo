function main(a: private bool, b: private bool) -> (u32) {
  if (a) {
    res = 1;
  } else if (b) {
    res = 2;
  } else {
    res = 3;
  }
  return res
}