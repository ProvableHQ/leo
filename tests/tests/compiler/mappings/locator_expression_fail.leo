
program relay.aleo {
    mapping users: address => bool;

    fn send(addr: address) -> Future {
        let a: u32 = relay.aleo/users;
        return finalize_send(addr);
    }
}

onchain fn finalize_send(addr: address) {
    let a:bool = Mapping::get(relay.aleo/users, addr);
}
