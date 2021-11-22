struct Foo {
    function echo(x: u32) -> u32 {
        return x;
    }
}

function main() {
    const err = Foo::echoed(1u32);
}