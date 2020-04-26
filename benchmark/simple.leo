from "./simple_import" import {
    Point as Foo,
    test
}

function main(a: private u32) -> (u32) {
  b = a + test()
  Foo c = Foo { x: 1, y: 2 }
  return b
}