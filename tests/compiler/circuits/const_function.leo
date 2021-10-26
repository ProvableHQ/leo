/*
namespace: Compile
expectation: Pass
input_file: input/dummy.in
*/

circuit Foo {
    x: u32

    const function ret_const(const x: u32) -> u32 {
        return x;
    }

    const function echo(const self) -> u32 {
        return self.x;
    }
}

function main(y: bool) -> bool {
    const a = Foo { x: 1u32 };

    return (a.echo() == Foo::ret_const(1u32)) == true;
}
