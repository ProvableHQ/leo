/*
namespace: Parse
expectation: Fail
*/

program relay.aleo {
    mapping users: address => bool;

    transition send(addr: address) -> Future {
        let a: mapping = relay.aleo/users;
        return finalize_send(addr);
    }
    async function finalize_send(addr: address) {
        let a:bool = Mapping::get(relay.aleo/users, addr);
    }
}
