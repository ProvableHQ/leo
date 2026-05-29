// FAIL: views are read-only. Every state-mutating operation must be inside a
// `final fn` or a `final { }` block. This case exercises all of them at once:
//   - singleton storage assignment (set + unset)
//   - `Mapping::set`, `Mapping::remove`
//   - `Vector::set`, `Vector::push`, `Vector::pop`, `Vector::swap_remove`, `Vector::clear`
program test.aleo {
    storage flag: u32;
    storage items: [u64];
    mapping balances: address => u64;

    fn dummy(x: u32) -> u32 { return x; }

    view fn poison(addr: address, val: u64) -> u32 {
        flag = 1u32;
        flag = none;

        balances.set(addr, val);
        balances.remove(addr);

        items.set(0u32, val);
        items.push(val);
        let popped = items.pop();
        let swapped = items.swap_remove(0u32);
        items.clear();
        assert(popped != none);
        assert(swapped >= 0u64);

        return 0u32;
    }

    @noupgrade
    constructor() {}
}
