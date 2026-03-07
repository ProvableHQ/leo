// Passing a u64 as a target argument to _dynamic_call should fail with the updated error message.
program test.aleo {
    fn main(prog: u64, net: field, func: identifier, arg: u64) -> u64 {
        let result: u64 = _dynamic_call::[public u64](prog, net, func, arg);
        return result;
    }
}
