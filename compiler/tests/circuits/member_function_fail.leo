circuit Foo {
    function echo(x: u32) -> u32 {
        return x
    }
}

function main() {
    let a = Foo { };
    let err = a.echoed(1u32);
}