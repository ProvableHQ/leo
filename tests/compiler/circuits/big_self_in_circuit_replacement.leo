/*
namespace: Compile
expectation: Pass
input_file: input/dummy.in
*/

circuit Foo {
  x: u32;

  function new() -> Self {
    let new: Self = Self {
      x: 1u32
    };

    return new;
  }

  function etc() {
      let y = [0u32, 1, 2, 3];
      y[Self {x: 0}.x] += 2;
  }
}

function main(y: bool) -> bool {
  let foo: Foo = Foo::new();
  return foo.x == 1u32 && y;
}