/*
namespace: Compile
expectation: Fail
*/

program test.aleo {    
    transition main(k: bool) -> bool {
    	let b: Foo = 1u8;
        return k == true;
    }}
