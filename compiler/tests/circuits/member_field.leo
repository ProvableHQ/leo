circuit Foo {
  x: u32,
}

function main() -> u32 {
  let c = Foo { x: 1u32 };

  return c.x
}