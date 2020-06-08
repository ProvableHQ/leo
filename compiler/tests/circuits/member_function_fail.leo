circuit Circ {
  function echo(x: u32) -> u32 {
    return x
  }
}

function main() -> u32 {
  let c = Circ { };
  return c.echoed(1u32)
}