circuit PedersenHash {
  parameters: group[1]

  static function new(parameters: group[1]) -> Self {
    return Self { parameters: parameters }
  }

}

function main() -> PedersenHash {
  let parameters = [0group; 1];
  let pedersen = PedersenHash::new(parameters);

  return pedersen
}