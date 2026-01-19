program simple.aleo {
    fn main(a: u16, b: u16) -> u32 {
        let (x, y) = foo(a as u32, b as u32);
        return x + y;
    }
}

fn foo(x: u32, y: u32) -> (u32, u32) {
    return (x, y);
}
