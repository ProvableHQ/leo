program test.aleo {
    fn main(a: u32, b: u32) -> u32 {
        let first: u32 = square(a);
        let second: u32 = square(b);
        return first + second;
    }
}

fn square(x: u32) -> u32 {
    return x * x;
}

