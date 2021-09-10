/*
namespace: Compile
expectation: Pass
input_file: input/dummy.in
*/

function main(y: bool) -> bool {
    let x = 0u8;
    for i in 0..strlen("I swear to god I had something for this") {
        x += 1;
    }
    return (x == 39) == y;
}

function strlen(str: [char; _]) -> u32 {
    return str.length();
}
