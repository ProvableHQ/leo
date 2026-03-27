interface Adder {
    fn sum(a: u32, b: u32) -> u32;
}

program test.aleo: Adder {
    fn sum(a: u32, b: u32) -> u32 {
        return a + b;
    }

    fn main(x: u32, y: u32) -> u32 {
        let target: identifier = 'foo';
        return Adder@(target)/sum(x, y);
    }
}
