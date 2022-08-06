/*
namespace: Compile
expectation: Pass
input_file:
 - ../inputs/dummy.in
*/

@program
function main(y: bool) -> bool {
    const a: i8 = 0i8;

    return (-a == 0i8) == y;
}
