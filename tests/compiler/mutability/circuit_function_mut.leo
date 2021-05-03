/*
namespace: Compile
expectation: Pass
*/

circuit Foo {
    x: u32

    function foo(mut self) {
        self.x = 10;
    }
}


function main() {
    let a = Foo { x: 1 };
    
    a.foo();

    console.assert(a.x == 10u32);
}