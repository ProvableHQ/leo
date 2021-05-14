/*
namespace: Compile
expectation: Fail
*/

function main() {
    const a = [1u8; 10];
    for i in 0..10 {
        const x = a[i..10];
        console.debug("{}", x);
    }
}
