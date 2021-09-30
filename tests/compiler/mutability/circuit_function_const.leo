/*
namespace: Compile
expectation: Fail
input_file: input/dummy.in
*/

circuit Foo {
    x: u32

    function foo(ref self) {
        self.x = 10;
    }
}

// cannot call mutable member function 'foo' of circuit 'Foo' from immutable context
function main() -> bool {
    const a = Foo { x: 1 };
    
    a.foo();

    return a.x == 10u32;
}
