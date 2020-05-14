circuit PedersenHash {
  static function new() {}
  parameters: group[1]

}

function main() {
  let parameters = [0group; 1];
  let pedersen = PedersenHash { parameters: parameters };
}