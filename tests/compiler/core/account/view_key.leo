/*
namespace: Compile
expectation: Fail
*/

// todo: rewrite this test so it properly tests a view key.
function main(public view_key: ViewKey, a: bool) -> bool {
    return a;
}
