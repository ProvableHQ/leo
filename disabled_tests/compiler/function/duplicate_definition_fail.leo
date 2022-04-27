/*
namespace: Compile
expectation: Fail
input_file: input/dummy.in
*/


function main(y: bool) -> bool {
    console.log("{}", 1u8);
    return y;
}

function main(y: bool) -> bool {
    console.log("{}", 2u8);
    return y; 
}
