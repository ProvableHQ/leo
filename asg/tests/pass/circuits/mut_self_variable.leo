circuit Foo {
    a: u8;

    function set_a(mut self, new: u8) {
        self.a = new;
        console.assert(self.a == new);
    }
}

function main() {
    let f  = Foo { a: 0u8 };

    console.assert(f.a == 0u8);

    f.set_a(1u8);

    console.assert(f.a == 1u8);

    f.set_a(2u8);

    console.assert(f.a == 2u8);
}