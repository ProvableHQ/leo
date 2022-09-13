/*
namespace: Compile
expectation: Fail
*/

@program
function matches(addr: address) -> bool {
    return self.foo == addr;
}
