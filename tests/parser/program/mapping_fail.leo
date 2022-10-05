/*
namespace: Parse
expectation: Fail
*/

program test.aleo {    
    mappin balances: address => u128;
    
    mapping foo: bar -> baz;
    
    mapping foo bar => baz;
}
