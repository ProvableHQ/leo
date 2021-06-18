circuit Foo {
  x: u32;

  function new() -> Self {
    let new: Self = Self {
      x: 1u32
    };

    return new;
  }

  function etc() {
     y[Self {x: 0}.func()] += 2;
  }
}

function main() {
  let foo: Foo = Foo::new();
}