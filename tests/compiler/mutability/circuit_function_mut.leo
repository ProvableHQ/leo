/*
namespace: Compile
expectation: Pass
input_file: input/dummy.in
*/

circuit Foo {
    x: u32

    function foo(ref self) {
        self.x = 10;
    }
}


function main(y: bool) -> bool {
    let a = Foo { x: 1 };
    
    a.foo();

    return (a.x == 10u32) == y;
}
