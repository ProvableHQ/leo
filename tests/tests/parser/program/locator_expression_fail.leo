
program relay.aleo {
    mapping users: address => bool;

    fn send(addr: address) -> Final {
        let a: mapping = relay.aleo/users;
        return final { finalize_send(addr); };
    }
}
final fn finalize_send(addr: address) {
    let a:bool = Mapping::get(relay.aleo/users, addr);
}
