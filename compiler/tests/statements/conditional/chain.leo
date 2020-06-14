function main(bit: u32) -> u32 {
  let mut result = 0u32;

  if bit == 1 {
    result = 1;
  } else if bit == 2 {
    result = 2;
  } else {
    result = 3;
  }

  return result
}