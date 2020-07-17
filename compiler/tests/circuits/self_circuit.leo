circuit Foo {
  static function new() -> Self {
    return Self { }
  }
}

function main() -> Foo {
  return Foo::new()
}