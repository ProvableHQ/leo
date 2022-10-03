/*
namespace: Compile
expectation: Fail
*/

transition matches(addr: address) -> bool {
    return self.foo == addr;
}
