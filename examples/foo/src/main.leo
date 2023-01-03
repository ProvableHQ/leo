// The 'foo' program.
program foo.aleo {
    transition main(public a: u32, b: scalar) -> u32 {
        let c: u32 = a + a;
        let aa: group = Pedersen64::commit(a, b);

        return c;
    }
}
