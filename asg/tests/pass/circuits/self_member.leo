circuit Foo {
    f: u32,

    function bar(self) -> u32 {
        return self.f
    }
}

function main() {
    let a = Foo { f: 1u32 };
    let b = a.bar();

    console.assert(b == 1u32);
}