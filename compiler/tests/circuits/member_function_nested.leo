circuit Foo {
    x: u32,

    function add_x(self, y: u32) -> u32 {
        return self.x + y
    }

    function call_add_x(self, y: u32) -> u32 {
        return self.add_x(y)
    }
}

function main() {
    const a = Foo { x: 1u32 };
    const b = a.add_x(1u32);

    console.assert(b == 2u32);
}
