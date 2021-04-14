circuit Foo {
  f: u8,
  y: (u8, u8),

  function z (mut self) -> u16 {
    self.y.0 += 1u8;
    return 1u16
  }
}
function main() {
  let x = 10u32;
  x += 20;
  console.assert(x == 30u32);

  let y = [1u8, 2u8];
  y[0] += 3u8;
  console.assert(y[0] == 4u8);

  let z = (1u8, 2u8);
  z.1 += 3u8;
  console.assert(z.1 == 5u8);

  let foo = Foo { f: 6u8, y: (1u8, 1u8) };
  foo.f += 2u8;
  console.assert(foo.f == 8u8);

  let a = [[0u8; 1]; 4];
  a[2][0] += 1u8;
  console.assert(a[2][0] == 1u8);

  let b = [0u8; (4, 1)];
  b[2][0] += 1u8;
  console.assert(a[2][0] == 1u8);
}