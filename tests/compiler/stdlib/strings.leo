/*
namespace: Compile
expectation: Pass
inputs:
 - hello.in: |
    [main]
    hello: [char; 5] = "hello";
    [registers]
    r0: bool = true;
*/

function main(hello: [char; 5]) -> bool {
    let hello2: string = "hello";

    return hello == hello2;
}