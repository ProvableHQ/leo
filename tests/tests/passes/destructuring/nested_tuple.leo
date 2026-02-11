program test.aleo {
    fn main(x: u32, y: u32) -> u32 {
        let pair: (u32, u32) = (x, y);
        let (a, b): (u32, u32) = pair;
        return a + b;
    }
}
