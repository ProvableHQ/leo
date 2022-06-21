/*
namespace: Compile
expectation: Pass
input_file: inputs/dummy.in
*/

function main (b: bool) -> bool {
    let x: u8 = 0u8;
    if false {
        return false;
    } else  {
        if false {
            return false;
        } else  {
            x = x;
            return (x == x) == b;
        }
    }
}