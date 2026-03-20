
interface Adder {
    fn sum(a: u32, b: u32) -> u32;
}

program test.aleo {
    fn main(target: identifier) -> u32 {
        return Adder@(target)/sum(1u32, 2u32);
    }
}
