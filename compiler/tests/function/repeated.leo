function test() -> bool {
  return true
}

function main() -> bool {
  return test() && test()
}