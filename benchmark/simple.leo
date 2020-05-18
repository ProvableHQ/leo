circuit Foo {
  x: u32

  static function new() -> Self {
    return Self { x: 5 }
  }
}

function main() {
  let a = Foo::new();
}