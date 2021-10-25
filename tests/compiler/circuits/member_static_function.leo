/*
namespace: Compile
expectation: Pass
input_file: input/dummy.in
*/

circuit Foo {
    static const N: u32 = 5;
    
    function echo(x: u32) -> u32 {
        return x;
    }

    function n_array() -> [char; Self::N] {
        return "hello";
    }
}

function main(y: bool) -> bool {
    let a = Foo::echo(1u32);

    return (a == 1u32) == y;
}
