function main(bit: u32) -> u32 {
  let mut result = 0u32;

  if bit == 1u32 {
    result = 1u32;
  } else if bit == 2u32 {
    result = 2u32;
  } else {
    result = 3u32;
  }

  return result
}