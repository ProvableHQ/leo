// Adding the `mut` keyword makes a circuit variable mutable.
circuit Foo {
    static function bar() {}
}

function main() {
    let mut a = Foo { x: 1 };
    a.bar = 0;
}