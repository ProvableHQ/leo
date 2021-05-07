circuit Foo {
    a: u8;

    function set_a(const self, new: u8) {
        self.a = new;
    }
}

function main() {
    let f  = Foo { a: 0u8 };

    f.set_a(1u8);
}