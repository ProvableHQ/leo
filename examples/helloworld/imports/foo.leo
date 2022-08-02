circuit Foo {
    x: u64,
    y: u64,
}

function bar(x: u64, y: u64) -> Foo {
    return Foo {x, y};
}