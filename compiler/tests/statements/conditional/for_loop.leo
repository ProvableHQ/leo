function main(cond: bool) -> u32 {
  let mut a = 0u32;

  if cond {
    for i in 0..4 {
      a += i;
    }
  }

  return a
}
