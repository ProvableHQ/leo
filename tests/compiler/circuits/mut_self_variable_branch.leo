/*
namespace: Compile
expectation: Pass
input_file: input/dummy.in
*/

circuit Foo {
    a: u8;

    function set_a(mut self, condition: bool, new: u8) {
        if condition {
            self.a = new;
        }
    }
}

function main(y: bool) -> bool {
    let f = Foo { a: 0u8 };

    f.set_a(false, 1u8);
    f.set_a(true, 1u8);
    f.set_a(false, 2u8);
    f.set_a(true, 2u8);

    return f.a == 2u8 == y;
}
