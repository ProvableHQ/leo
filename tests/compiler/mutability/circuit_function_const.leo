/*
namespace: Compile
expectation: Fail
*/

circuit Foo {
    x: u32

    function foo(mut self) {
        self.x = 10;
    }
}

// cannot call mutable member function 'foo' of circuit 'Foo' from immutable context
function main() {
    const a = Foo { x: 1 };
    
    a.foo();

    console.assert(a.x == 10u32);
}