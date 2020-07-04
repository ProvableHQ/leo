circuit Foo {
    x: u32,

    function add_x(y: u32) -> u32 {
        return self.x + y
    }

    function call_add_x(y: u32) -> u32 {
        return self.add_x(y)
    }
}

function main() -> u32 {
    let f = Foo { x: 1u32 };

    return f.call_add_x(1u32)
}
