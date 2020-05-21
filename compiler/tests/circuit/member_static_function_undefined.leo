circuit Circ {
  static function echo(x: u32) -> u32 {
    return x
  }
}

function main() -> u32 {
  return Circ::echoed(1u32)
}