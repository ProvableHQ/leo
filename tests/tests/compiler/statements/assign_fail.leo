/*
namespace: Compile
expectation: Fail
*/

program test.aleo {    
    x::y = y;
    
    5 = y;
    
    x + x = y;
    
    -x = y;
    
    !x = y;
    
    a? x : x = y;
    
    x as u32 = y;
    
    [x, x, x] = y;
    
    [x; 3] = y;
    
    (x, x, x) = y;
    
    x {x: y, y: z} = y;
    
    x() = y;
    
    x.y() = y;
    
    🦀 = y;
}
