program test.aleo {
    fn main(x: u32, y: u32) -> u32 {
        let result: u32 = add(x, y);
        return result;
    }
}

fn add(a: u32, b: u32) -> u32 {
    return a + b;
}
