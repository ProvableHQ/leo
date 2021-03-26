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
}