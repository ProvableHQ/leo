/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    transition main(public a: u32, b: u32) -> u32 {
        let c: u32 = a + b;
        return c;
    }
    
    struct Board {
       foo: u8,
    }
    
    function win(b: Board, p: u8) -> bool {
    	return false & true;
    }
}
