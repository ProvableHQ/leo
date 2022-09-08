/*
namespace: Compile
expectation: Pass
*/

@program
function baz(a: (u8, u8)) -> u8 {
    let result: u8 = a.0 + a.1;
    return result;
}
