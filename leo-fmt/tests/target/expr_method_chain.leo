struct Wrapper {
    inner: u64,
}

program test.aleo {
    mapping balances: address => u64;

    fn deposit(public amount: u64) -> Final {
        let caller: address = self.caller;
        let signer: address = self.signer;
        return final {
            finalize_deposit(caller, amount);
        };
    }

    fn access_chain(w: Wrapper, arr: [u64; 3], x: u64, y: u64) -> u64 {
        let a: u64 = w.inner;
        let b: u64 = arr[0u32];
        let tup: (u64, u64) = (x, y);
        let c: u64 = tup.0;
        let d: u64 = tup.1;
        return a;
    }
}

final fn finalize_deposit(sender: address, amount: u64) {
    let bal: u64 = balances.get(sender);
    let check: bool = balances.contains(sender);
    balances.set(sender, amount);
    balances.remove(sender);
    let val: u64 = balances.get_or_use(sender, 0u64);
    let bal2: u64 = Mapping::get(balances, sender);
    Mapping::set(balances, sender, amount);
    Mapping::remove(balances, sender);
    let val2: u64 = Mapping::get_or_use(balances, sender, 0u64);
    let h: u32 = block.height;
    let t: i64 = block.timestamp;
}
