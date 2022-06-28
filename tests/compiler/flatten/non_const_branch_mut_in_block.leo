/*
namespace: Compile
expectation: Skip
input_file: inputs/dummy.in
*/

function main (b: bool) -> bool {
    let x: u8 = 0u8;
    {
        let y: u8 = x;
        if b {
            x = y;
            y = y;
        }
    }
    {
        let y: u8 = x;
        if b {
            x = y;
            y = y;
        }
    }
    return x == 0u8;
}