/*
namespace: Parse
expectation: Fail
*/

program relay.aleo {
    mapping users: address => bool;

    transition send(addr: address) {
        let a: mapping = relay.aleo/users;
        return then finalize(addr);
    }
    finalize send(addr: address) {
        let a:bool = Mapping::get(relay.aleo/users, addr);
    }
}