circuit Circ {
  x: u32
}

function main() -> u32 {
  let mut a = 1;
  a = 0;

  let b = 1;
  //b = 0; // <- illegal

  let mut arr = [1, 2];
  arr[0] = 0;

  let mut c = Circ { x: 1 };
  c.x = 0;

  return c.x
}