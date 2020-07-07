circuit Foo {
    function bar() -> u32 {
        return self.f
    }
}

function main() -> u32 {
    let circuit = Foo { };
    return circuit.bar()
}