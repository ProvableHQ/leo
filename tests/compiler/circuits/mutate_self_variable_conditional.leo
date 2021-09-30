/*
namespace: Compile
expectation: Pass
input_file: input/dummy.in
*/

function main(y: bool) -> bool {
    let f = Foo { a: 0u32 };

    f.bar();

    return y == true;
}

circuit Foo {
    a: u32;

    function bar(ref self) {
        if true {
            self.a = 5u32; // Mutating a variable inside a conditional statement should work.
        }
    }
}
