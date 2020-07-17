// Circuits are immutable by default.
circuit Foo {
  x: u32
}

function main() {
  let a = Foo { x: 1 };
  a.x = 0;
}