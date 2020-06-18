// Multidimensional array syntax in leo
function main() -> u32[3][2] {
  const m = [[0u32, 0u32, 0u32], [0u32, 0u32, 0u32]]; // inline

  const m: u32[3][2] = [[0; 3]; 2]; // initializer

  return m
}