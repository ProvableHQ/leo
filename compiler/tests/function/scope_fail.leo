function foo() -> field {
  return myGlobal
}

function main() -> field {
  let myGlobal = 42field;
  return foo()
}