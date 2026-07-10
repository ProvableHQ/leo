export interface Sink {
    fn consume(a: [u8; 4]) -> u32;
}

program test.aleo: Sink {
    fn consume(a: [u8; 4]) -> u32 {
        return a[0u8] as u32;
    }

    // Regression: an array written element-wise then passed to a dynamic call must be rebuilt from its writes.
    fn driver(target: field, public p0: u8, public p1: u8) -> u32 {
        let arr: [u8; 4] = [0u8; 4];
        for i: u32 in 0u32..4u32 {
            arr[i] = p0 + p1;
        }
        return Sink@(target)::consume(arr);
    }

    @noupgrade
    constructor() {}
}
