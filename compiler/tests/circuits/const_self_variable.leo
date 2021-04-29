circuit Foo {
    a: u8;

    function use_a(const self) -> u8 {
        return self.a + 1;
    }
}

function main() {
    let f  = Foo { a: 0u8 };

    console.assert(1u8 == f.use_a());
}