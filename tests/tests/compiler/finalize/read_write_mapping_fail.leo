
program test.aleo {
    mapping balances: address => u128;

    transition read(addr: address) -> public u128 {
        return balances[addr];
    }

    transition write(addr: address, public amount: u128) {
        balances[addr] = amount;
    }
}
