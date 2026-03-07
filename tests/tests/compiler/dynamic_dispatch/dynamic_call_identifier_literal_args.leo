// _dynamic_call with inline identifier literals as program/network/function targets.
program test.aleo {
    fn main(arg: u64) -> u64 {
        let result: u64 = _dynamic_call::[public u64]('some_program', 'aleo', 'transfer', arg);
        return result;
    }
}
