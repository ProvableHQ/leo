circuit Foo {
    function bar() -> u32 {
        return self.f
    }
}

function main() {
    let foo = Foo { };
    let err = foo.bar();
}