interface Adder {
    fn sum(a: u32, b: u32) -> u32;
}

interface Counter {
    storage total: u64;
    storage entries: [u64];
    mapping balances: address => u64;
}

program test.aleo {
    fn main(target: identifier, key: address, i: u32) -> u32 {
        return Adder@(target)::sum(1u32, 2u32);
    }

    fn finalize_read(target: identifier, key: address, i: u32) -> Final {
        return final {
            let a: u64? = Counter@(target)::balances.get(key);
            let b: u64? = Counter@(target, 'aleo')::total;
            let c: u64? = Counter@(target)::entries.get(i);
            let d: u32 = Counter@(target)::entries.len();
        };
    }
}
