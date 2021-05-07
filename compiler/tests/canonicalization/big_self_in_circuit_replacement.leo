circuit Foo {
  x: u32;

  function new() -> Self {
    let new: Self = Self {
      x: 1u32
    };

    return new;
  }
}

function main() {
  let foo: Foo = Foo::new();
}