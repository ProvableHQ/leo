circuit Foo {
    x: u32,

    function add_x(y: u32) -> u32 {
        return self.x + y
    }

    function call_add_x(y: u32) -> u32 {
        return self.add_x(y)
    }
}

function main() {
    let a = Foo { x: 1u32 };
    let b = a.call_add_x(1u32);

    assert_eq!(b, 2u32);
}
