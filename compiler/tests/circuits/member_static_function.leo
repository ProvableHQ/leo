circuit Foo {
    static function echo(x: u32) -> u32 {
        return x
    }
}

function main() {
    let a = Foo::echo(1u32);

    assert_eq!(a, 1u32);
}