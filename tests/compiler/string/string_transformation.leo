/*
namespace: Compile
expectation: Pass
input_file: inputs/two.in
*/

function main(s2: [char; 2]) -> bool {
    let s = "\u{2764}ello, World!\u{DDDD}";
    s[..2] = s2;
    let x = "test1";
    let z = [1u8, 2u8, 3u8, 4u8];
    z[0.."test" == "test" ? 2 : 2] = [10u8, 10];
    return z == [10u8, 10, 3, 4] && s == "hello, World!\u{DDDD}";
}
