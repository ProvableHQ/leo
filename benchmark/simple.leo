circuit PedersenHash {

  parameters: group[1]

  static function new(parameters: group[1]) -> Self {
    return Self { parameters: parameters }
  }

  function hash(bits: bool[1]) -> group {
    let mut digest: group = 0group;

    for i in 0..1 {
      let base: group = if bits[i] ? parameters[i] : 0group;
      digest += base;
    }

    return digest
  }
}

function main() -> group {
  let parameters = [0group; 1];
  let pedersen = PedersenHash::new(parameters);

  let input: bool[1] = [true];
  let output = pedersen.hash(input);

  return output
}