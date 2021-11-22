function main() {
    let f = Foo { a: 0u32 };

    f.bar();
}

struct Foo {
    a: u32;

    function bar(&self) {
        if true {
            self.a = 5u32; // Mutating a variable inside a conditional statement should work.
        }
    }
}