/*
namespace: Compile
expectation: Pass
input_file:
 - input/complex_access.in
*/

function main (a: [u8; 8], b: [[u8; 3]; 3], c: [(u8, u32); 1]) -> bool {
  a[2..6][1] = 87;
  a[2..3] = [42u8];
  a[6..][0] = 43u8;

  b[0..2][0] = [1u8; 3];
  b[1..][1][1..2][0] = 126;
  b[1..][0] = [42, 43, 44];

  c[..1][0].1 = 1;

  return
    a == [1u8, 2, 42, 87, 5, 6, 43, 8]
    && b == [[1u8, 1, 1], [42, 43, 44], [7, 126, 9]]
    && c == [(0u8, 1u32)];
}