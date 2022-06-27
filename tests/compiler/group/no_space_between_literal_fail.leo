/*
namespace: Compile
expectation: Fail
*/

function main() -> group {
  const g: group = (0,1) group;
  return g;
}