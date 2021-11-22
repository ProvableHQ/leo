/*
namespace: Compile
expectation: Fail
input_file: input/dummy.in
*/

struct Foo {
    a: u8;

    function set_a(mut self, new: u8) {
        self.a = new;
    }
}

function main(y: bool) -> bool {
    let f = Foo { a: 0u8 };

    f.set_a(1u8);
    f.set_a(2u8);

    return (f.a == 2u8) == y;
}
