interface Adder {
    fn sum(a: u32, b: u32) -> u32;
}

program test.aleo: Adder {
    fn sum(a: u32, b: u32) -> u32 {
        return a + b;
    }

    fn main(x: u32, y: u32) -> u32 {
        return Adder@('foo')/sum(x, y);
    }
}
