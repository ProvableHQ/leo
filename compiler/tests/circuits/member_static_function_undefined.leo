circuit Foo {
    function echo(x: u32) -> u32 {
        return x
    }
}

function main() {
    let err = Foo::echoed(1u32);
}