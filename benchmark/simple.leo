circuit PedersenHash {
  parameters: group[1]

  function new(b: u32) -> u32 {
    return b
  }
}

function main() -> u32{
  let parameters = [0group; 1];
  let mut pedersen = PedersenHash { parameters: parameters };
  let b = PedersenHash::new(3);

  return b
}