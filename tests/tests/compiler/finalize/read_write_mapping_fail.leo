
program test.aleo {
    mapping balances: address => u128;

    mapping misc: address => u128;

    fn read(addr: address) -> public u128 {
        return balances[addr];
    }

    fn write(addr: address, public amount: u128) {
        balances[addr] = amount;
    }

    fn assign(addr: address, public amount: u128) {
        balances = misc;
    }
}
