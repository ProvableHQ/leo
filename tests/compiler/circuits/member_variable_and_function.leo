/*
namespace: Compile
expectation: Pass
inputs:
 - member.in: |
    [registers]
    r0: bool = false;  
*/

circuit Foo {
    foo: u32;

    function bar() -> u32 {
        return 1u32;
    }
}

function main() -> bool {
    const a = Foo { foo: 1 };
    const b = a.foo + Foo::bar();

    return b == 2u32;
}
