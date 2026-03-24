// FAIL: Interface@() dynamic calls cannot be used inside a conditional.
interface Adder {
    fn sum(a: u32, b: u32) -> u32;
}

program test.aleo : Adder {
    fn sum(a: u32, b: u32) -> u32 {
        return a + b;
    }

    fn main(target: field, x: u32, y: u32, cond: bool) -> u32 {
        if cond {
            return Adder@(target)/sum(x, y);
        }
        return 0u32;
    }
}
