circuit Foo {
    static function new() -> Self {
        return Self { }
    }
}

function main() {
    let a = Foo::new();
}