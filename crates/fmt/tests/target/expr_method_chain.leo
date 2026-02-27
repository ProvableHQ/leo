struct Wrapper {
    inner: u64,
}

final fn use_special(caller_addr: address) {
    let h: u32 = block.height;
    let t: i64 = block.timestamp;
    let n: u16 = network.id;
}

final fn do_deposit(sender: address, amount: u64) {
    let bal: u64 = balances.get(sender);
    let check: bool = balances.contains(sender);
    balances.set(sender, amount);
    balances.remove(sender);
    let val: u64 = balances.get_or_use(sender, 0u64);
}

fn access_chain(w: Wrapper, arr: [u64; 3], tup_a: u64, tup_b: u64) -> u64 {
    let a: u64 = w.inner;
    let b: u64 = arr[0u32];
    let c: u64 = tup_a;
    let d: u64 = tup_b;
    return a;
}

program test.aleo {
    mapping balances: address => u64;

    fn deposit(public sender: address, public amount: u64) -> Final {
        let caller: address = self.caller;
        let signer: address = self.signer;
        return final {
            do_deposit(sender, amount);
        };
    }
}
