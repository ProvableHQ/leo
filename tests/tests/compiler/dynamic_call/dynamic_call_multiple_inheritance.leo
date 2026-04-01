interface Adder {
    fn sum(a: u32, b: u32) -> u32;
}

interface Multiplier {
    fn product(a: u32, b: u32) -> u32;
}

interface Calculator: Adder + Multiplier {
}

program test.aleo: Calculator {
    fn sum(a: u32, b: u32) -> u32 {
        return a + b;
    }

    fn product(a: u32, b: u32) -> u32 {
        return a * b;
    }

    fn main(target: field, x: u32, y: u32) -> u32 {
        let s: u32 = Adder@(target)::sum(x, y);
        let p: u32 = Multiplier@(target)::product(x, y);
        return s + p;
    }
}
