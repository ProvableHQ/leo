// Pass a dyn record through a dynamic call: receive it and send it back.
program test.aleo {
    fn main(prog: field, net: field, func: field, token: dyn record) -> dyn record {
        let result: dyn record = _dynamic_call::[dyn record, dyn record](prog, net, func, token);
        return result;
    }
}
