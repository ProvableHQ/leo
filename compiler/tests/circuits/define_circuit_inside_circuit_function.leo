circuit Foo {
    a: u32,
}

circuit Bar {
    function bar() {
        let f = Foo { a: 0u32 };
    }
}

function main() {
    let b = Bar::bar();
}