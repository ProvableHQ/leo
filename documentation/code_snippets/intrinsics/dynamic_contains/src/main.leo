program checker.aleo {
    mapping seen: address => bool;

    fn check(prog: field, net: field, map: field, key: address) -> Final {
        return final { finalize_check(prog, net, map, key); };
    }

    @noupgrade
    constructor() {}
}

final fn finalize_check(prog: field, net: field, map: field, key: address) {
    let exists: bool = _dynamic_contains(prog, net, map, key);
    Mapping::set(seen, key, exists);
}
