circuit Foo {
    foo: u32,

    static function bar() -> u32 {
        return 1u32
    }
}

function main() {
    let a = Foo { foo: 1 };

    let b = a.foo + Foo::bar();

    assert_eq!(b, 2u32);
}
