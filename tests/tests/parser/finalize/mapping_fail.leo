/*
namespace: Parse
expectation: Fail
*/

program test.aleo {    
    mapping u8: u8 => u8;
    
    mapping foo: bar -> baz;
    
    mapping foo: (bar => baz);
    
    mapping foo: [bar -> baz];
    
    mapping foo bar => baz;
}
