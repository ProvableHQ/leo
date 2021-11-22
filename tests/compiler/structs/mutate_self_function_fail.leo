/*
namespace: Compile
expectation: Fail
*/

struct Foo {
    a: u8;

    function bar() {}

    function set_a(&self, new: u8) {
        self.bar = new;
    }
}

function main() {
    let f = Foo { a: 0u8 };

    f.set_a(1u8);
}
