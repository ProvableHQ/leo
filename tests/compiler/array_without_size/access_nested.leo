/*
namespace: Compile
expectation: Pass
input_file: input/dummy.in
*/

function main(y: bool) -> bool {
    let d: [u8; (_, _)] = [[1,2,3,4], [1,2,3,4]];
    return (d[0][0] == 1) == y;
}
