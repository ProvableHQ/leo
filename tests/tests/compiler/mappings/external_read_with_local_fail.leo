/*
namespace: Compile
expectation: Pass
*/

program registry.aleo {
    mapping users: address => bool;

    transition register() {
        return then finalize(self.caller);
    }
    finalize register(addr: address) {
        Mapping::set(users, addr, true);
    }
}

// --- Next Program --- //

program relay.aleo {
    mapping users: address => bool;

    function send(addr: address) {
        return then finalize(addr);
    }
    finalize send(addr: address) {
        let a:bool = Mapping::get(relay.aleo/users, r0);
    }
}
