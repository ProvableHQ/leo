/*
namespace = "Compile"
expectation = "Fail"
*/

program test.aleo {    
    function main(x: bool) -> group {
      let g: group = (0,1) group;
      return g;
    }}
