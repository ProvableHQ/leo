/*
namespace: Compile
expectation: Pass
*/

circuit Foo {
    a: u8,

    function set_a(mut self, condition: bool, new: u8) {
        if condition {
            self.a = new;
            console.assert(self.a == new);
        }
    }
}

function main() {
    let f = Foo { a: 0u8 };

    console.assert(f.a == 0u8);

    f.set_a(false, 1u8);

    console.assert(f.a == 0u8);

    f.set_a(true, 1u8);

    console.assert(f.a == 1u8);

    f.set_a(false, 2u8);

    console.assert(f.a == 1u8);

    f.set_a(true, 2u8);

    console.assert(f.a == 2u8);
}
