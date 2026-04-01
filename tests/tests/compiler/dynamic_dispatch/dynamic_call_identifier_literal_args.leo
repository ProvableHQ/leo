// _dynamic_call with identifier literal arguments.
program test.aleo {
    fn main(x: u64) -> u64 {
        let result: u64 = _dynamic_call::[u64]('some_program', 'aleo', 'add', x);
        return result;
    }
}
