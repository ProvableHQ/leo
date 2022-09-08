/*
namespace: Compile
expectation: Pass
*/

@program
function baz(a: (u8, u16)) -> u8 {
    a = (3u8, 4u16);
    return 1u8 + 1u8;
}
