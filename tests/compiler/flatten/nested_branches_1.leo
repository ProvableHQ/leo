/*
namespace: Compile
expectation: Pass
input_file: inputs/dummy.in
*/

function main (b: bool) -> bool {
    let x: u8 = 0u8;
    if true {
        if true {
            x = x;
            return (x == x) == b;
        } else  {
            return false;
        }
    } else  {
        return false;
    }
}