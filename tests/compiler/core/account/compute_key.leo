/*
namespace: Compile
expectation: Fail
*/

// todo: rewrite this test so it properly tests a compute key.
function main(public compute_key: ComputeKey, a: bool) -> bool {
    return a;
}
