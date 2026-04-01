// _dynamic_call with multiple input type annotations and tuple return.
program test.aleo {
    fn main(target: field, net: field, func: field, x: u32, y: u32) -> (u32, u32) {
        let (a, b): (u32, u32) = _dynamic_call::[public u32, public u32, (u32, u32)](target, net, func, x, y);
        return (a, b);
    }
}
