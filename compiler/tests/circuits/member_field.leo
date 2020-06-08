circuit Circ {
  x: u32
}

function main() -> u32 {
  let c = Circ { x: 1u32 };

  return c.x
}