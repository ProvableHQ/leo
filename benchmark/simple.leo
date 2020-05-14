circuit PedersenHash {
  parameters: group[1]

  static function new(b: u32) -> u32 {
    return b
  }
}

function main() -> u32{
  let parameters = [0group; 1];
  let pedersen = PedersenHash { parameters: parameters };
  let b = PedersenHash::new(3);

  return b
}