/*
namespace: Compile
expectation: Fail
*/

circuit Foo {
    const function set_a(a: b) {
        self.a = new;
    }
}

function main() {
    let f  = Foo { a: 0u8 };

    f.set_a(1u8);
}
