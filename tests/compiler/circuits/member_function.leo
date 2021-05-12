/*
namespace: Compile
expectation: Pass
input_file: input/dummy.in
*/

circuit Foo {
    x: u32,

    function echo(self) -> u32 {
        return self.x;
    }
}

function main(y: bool) -> bool {
    const a = Foo { x: 1u32 };

    return a.echo() == 1u32 == true;
}
