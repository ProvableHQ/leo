/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    transition main(x: address) -> bool {
        const sender: address = aleo1l7ytv5jqjzpxtjqttl5z9mle8ujcpac9t6tkge5f4haah4pxas8sagzecd;
        const receiver: address = aleo1dtpkpg3d653mdlzh6g028937qdgujecn5gw5tzh7ftcvyz7jxvfqw6t8p6;
    
        return x == sender ? receiver == x : sender == x;
    }}
