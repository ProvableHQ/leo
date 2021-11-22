struct Foo {
    static function new() -> Self {
        return Self { };
    }
}

function main() {
    const a = Foo::new();
}