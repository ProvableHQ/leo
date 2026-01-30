
program test.aleo {
    fn main(a: u32, b: u32, y: bool) -> u32 {
        if y {
           return adder(a, b);
        } else {
            return subber(a, b);
        }
    }

    fn adder(a: u32, b: u32) -> u32 {
        return a + b;
    }

    fn subber(a: u32, b: u32) -> u32 {
        return a - b;
    }}
