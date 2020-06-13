function main(bit: private u32) -> u32 {
  let mut a = 5u32;

  if bit == 1u32 {
    a = 1u32;
  } else {
    a = 0u32;
  }

  return a
}
