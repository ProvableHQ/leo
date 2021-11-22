/*
namespace: Compile
expectation: Fail
input_file: input/dummy.in
*/

struct Foo {
  x: u32;

  function new() -> Self {
    let new: Self = Self {
      x: 1u32
    };

    return new;
  }
}

function main() {
  let foo: Self = Foo::new();
}