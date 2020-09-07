circuit Foo {
    mut a: u8,

    function set_a(new: u8) {
        self.a = new;
    }
}

function main() {
    let mut f  = Foo { a: 0u8 };

    console.assert(f.a == 0u8);

    f.set_a(1u8);

    console.assert(f.a == 1u8);

    f.set_a(2u8);

    console.assert(f.a == 2u8);
}