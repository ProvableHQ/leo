// Circuits are immutable by default.
circuit Circ {
  x: u32
}

function main() {
  let a = Circ { x: 1 };
  a.x = 0;
}