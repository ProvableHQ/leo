/*
namespace: Compile
expectation: Fail
*/

// todo: rewrite this test so it properly tests a record type.
function main(public record: Record, a: bool) -> bool {
    return a;
}
