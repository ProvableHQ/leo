/*
namespace: Compile
expectation: Pass
input_file: input/dummy.in
*/

program test.aleo {    
    struct Foo {
        x: u32,
    }
    
    transition main(y: bool) -> bool {
        const a: Foo = Foo { x: 1u32 };
    
        return (a.x == 1u32) == y;
    }
}
