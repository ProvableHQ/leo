/*
namespace: Compile
expectation: Fail
*/

// todo: rewrite this test so it properly tests a private key.
function main(public private_key: PrivateKey, a: bool) -> bool {
    return a;
}
