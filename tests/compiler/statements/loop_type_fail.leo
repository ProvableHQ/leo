/*
namespace: Compile
expectation: Fail
input_file: inputs/dummy.in

# inclusive loops are currently disallowed

*/

function main () -> bool {
    for i: bool in true..false {
        let a: bool = i;
    }
    return true;
}