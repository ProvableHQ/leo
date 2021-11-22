/*
namespace: Compile
expectation: Fail
*/

struct Foo {
    a: u8,
    const function set_a(self, a: u8) {
        self.a = a;
    }
}

function main() {
    let f  = Foo { a: 0u8 };

    f.set_a(1u8);
}
