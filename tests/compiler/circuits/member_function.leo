/*
namespace: Compile
expectation: Pass
inputs:
 - member.in: |
    [registers]
    r0: bool = false;  
*/

circuit Foo {
    x: u32,

    function echo(self) -> u32 {
        return self.x;
    }
}

function main() -> bool {
    const a = Foo { x: 1u32 };

    return a.echo() == 1u32;
}
