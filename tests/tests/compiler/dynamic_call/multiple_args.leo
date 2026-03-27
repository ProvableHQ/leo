interface Processor {
    fn process(a: address, amount: u64) -> bool;
}

program test.aleo: Processor {
    fn process(a: address, amount: u64) -> bool {
        return amount > 0u64;
    }

    fn main(target: field, addr: address, val: u64) -> bool {
        return Processor@(target)/process(addr, val);
    }
}
