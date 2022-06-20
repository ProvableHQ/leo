/*
namespace: Compile
expectation: Fail
*/

// todo: rewrite this test so it properly tests a signature.
function main(public signature: Signature, a: bool) -> bool {
    return a;
}
