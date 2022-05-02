/*
namespace: Compile
expectation: Pass
input_file: inputs/dummy.in
*/

circuit Foo {
  f: u8;
  y: (u8, u8);

  function z (mut self) -> u16 {
    self.y.0 += 1u8;
    return 1u16;
  }
}

function main(k: bool) -> bool {
  let x = 10u32;
  x += 20;

  let w = 3u32;
  w += x;

  let y = [1u8, 2u8, 3, 4];
  y[0] += 3u8;
  y[0..3][1] *= 3;

  let z = (1u8, 2u8);
  z.1 += 3u8;

  let foo = Foo { f: 6u8, y: (1u8, 1u8) };
  foo.f += 2u8;

  let complex = 2u8;
  complex += 22u8 - 2u8+ 1u8;
  let a = [[0u8; 1]; 4];
  a[2][0] += 1u8;

  let b = [0u8; (4, 1)];
  b[2][0] += 1u8;

  return x == 30u32 && w == 33u32 && y[0] == 4u8 && y[1] == 6u8
  && z.1 == 5u8 && foo.f == 8u8 && a[2][0] == 1u8 && a[2][0] == 1u8
  && k;
}