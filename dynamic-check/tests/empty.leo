circuit Foo {
    a: u8,
    b: u8,

    static function one() -> u8 {
        return 1
    }
}

function main(mut a: u8) -> bool {
    a = Foo::one();
    let b = true;
    return b
}