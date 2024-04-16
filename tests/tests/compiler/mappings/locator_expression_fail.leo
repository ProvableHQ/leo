/*
namespace: Compile
expectation: Fail
*/

program relay.aleo {
    mapping users: address => bool;

    async transition send(addr: address) -> Future {
        let a: u32 = relay.aleo/users;
        return finalize_send(addr);
    }
    async function finalize_send(addr: address) {
        let a:bool = Mapping::get(relay.aleo/users, addr);
    }
}
