// Adding the `mut` keyword makes a circuit variable mutable.
circuit Circ {
  x: u32
}

function main() -> u32 {
  let mut a = Circ { x: 1 };
  a.x = 0;

  return a.x
}