function test() -> (bool, bool) {
  return (true, false)
}

function main() -> (bool, bool) {
  let (a, b) = test();
  return (a, b)
}