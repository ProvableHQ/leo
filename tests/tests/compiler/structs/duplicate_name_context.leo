/*
namespace: Compile
expectation: Fail
*/

program test.aleo {    
    struct Bar {
        b2: u32
    }
    
    function main(y: bool) -> bool {
        let Bar: u32 = 66u32;
        let k1: Bar = Bar { b2: 30u32 };
    
        return y == true;
    }
}
