interface Adder {
    fn sum(a: u32, b: u32) -> u32;
}

program test.aleo : Adder {
    fn sum(a: u32, b: u32) -> u32 {
        return a + b;
    }

    fn main(target: field) -> u32 {
        return Adder@(target)/sum(1u32, true);
    }
}
