/*
namespace: Compile
expectation: Fail
*/

function main() {
    let x = 3u8;
    const y = x > 2u8? 1u8 : 2u8;
}
