program reader.aleo {
    mapping result: address => u64;

    fn read(prog: field, net: field, map: field, key: address) -> Final {
        return final { finalize_read(prog, net, map, key); };
    }

    @noupgrade
    constructor() {}
}

final fn finalize_read(prog: field, net: field, map: field, key: address) {
    let val: u64 = _dynamic_get::[u64](prog, net, map, key);
    Mapping::set(result, key, val);
}
