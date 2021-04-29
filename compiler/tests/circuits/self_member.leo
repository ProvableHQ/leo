circuit Foo {
    f: u32;

    function bar(self) -> u32 {
        return self.f;
    }
}

function main() {
    const a = Foo { f: 1u32 };
    const b = a.bar();

    console.assert(b == 1u32);
}