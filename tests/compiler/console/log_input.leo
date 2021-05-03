/*
namespace: Compile
expectation: Pass
inputs: 
 - log.in: |
    [main]
    a: bool = true;
*/

function main(a: bool) {
    console.log("a = {}", a);
}