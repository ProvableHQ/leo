circuit Foo {
    f: u32,

    function bar() -> u32 {
        return self.f
    }
}

function main() -> u32 {
    let circuit = Foo { f: 1u32 };
    return circuit.bar()
}