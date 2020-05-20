// Multidimensional array syntax in leo
function main() -> u32[3][2] {
  let m = [[0u32, 0u32, 0u32], [0u32, 0u32, 0u32]]; // inline

  let m: u32[3][2] = [[0; 3]; 2]; // initializer

  return m
}