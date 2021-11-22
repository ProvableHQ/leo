/*
namespace: Compile
expectation: Pass
input_file: input/dummy.in
*/

struct Foo {
    a: u8;

    function use_a(const self) -> u8 {
        return self.a + 1;
    }
}

function main(y: bool) -> bool {
    let f  = Foo { a: 0u8 };

    return (1u8 == f.use_a()) == true;
}
