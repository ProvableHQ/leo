/*
namespace: Parse
expectation: Fail
*/

program test.aleo {    
    function main(s1: [char; 13], s2: [char; 4]) -> bool {
        let d: char = '\u}
