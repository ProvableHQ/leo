/*
namespace: Compile
expectation: Pass
input_file: input/dummy.in
*/

circuit Foo {
    x: u32;

    function new() -> Self {
        let new: Self = Self {
            x: 1u32
        };

        return new;
    }

    function etc() {
        let y = [0u32, 1, 2, 3];
        y[Self {x: 0}.x] += 2;
    }

    function func() {
        const x: Self = Foo {x: Self {x: 1}.x};
    }

    function self_circ(x: Self) {}
}

function main(y: bool) -> bool {
    let foo: Foo = Foo::new();
    return foo.x == 1u32 && y;
}