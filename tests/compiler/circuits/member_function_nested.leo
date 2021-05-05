/*
namespace: Compile
expectation: Pass
inputs:
 - inline.in: |
    [main]
    x: u32 = 10;

    [registers]
    r0: bool = false;  
*/

circuit Foo {
    x: u32,

    function add_x(self, y: u32) -> u32 {
        return self.x + y;
    }

    function call_add_x(self, y: u32) -> u32 {
        return self.add_x(y);
    }
}

function main(x: u32) -> bool {
    const a = Foo { x };
    const b = a.add_x(1u32);

    return b == x + 1;
}
