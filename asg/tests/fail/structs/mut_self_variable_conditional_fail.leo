function main() {
    let f = Foo { a: 0u32 };

    f.bar();
}

struct Foo {
    a: u32;

    function bar(mut self) {
        if true {
            self.a = 5u32;
        }
    }
}