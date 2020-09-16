circuit Foo {
    a: u8,

    static function bar() {}

    function set_a(new: u8) {
        self.bar = new;
    }
}

function main() {
    let mut f  = Foo { a: 0u8 };

    f.set_a(1u8);
}