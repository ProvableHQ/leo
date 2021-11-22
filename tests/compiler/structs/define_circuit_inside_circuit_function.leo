/*
namespace: Compile
expectation: Pass
inputs:
 - func_circ.in: |
    [main]
    a: u32 = 100;

    [registers]
    r0: bool = false;
*/

struct Foo {
    a: u32,
}

struct Bar {
    function bar(a: u32) -> u32 {
        let f = Foo { a: a };
        return f.a;
    }
}

function main(a: u32) -> bool {
    let b = Bar::bar(a);
    return a == b;
}
