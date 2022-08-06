/*
namespace: Compile
expectation: Pass
input_file:
 - ../inputs/dummy.in
*/

@program
function main(y: bool) -> bool {
    const a: i16 = 0i16;

    return (-a == 0i16) == y;
}
