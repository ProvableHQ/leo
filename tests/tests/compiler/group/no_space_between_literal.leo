/*
namespace: Compile
expectation: Fail
*/

program test.aleo {    
    function main() -> group {
      const g: group = (0,1) group;
      return g;
    }}
