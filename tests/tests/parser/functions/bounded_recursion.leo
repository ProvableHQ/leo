
program test.aleo {}

fn x(constant y: u32) -> u8 {
    if y < 5u32 {
        x(y+1u32);
    }
}

fn main(y: bool) -> bool {
    x(1u32);
    return y;
}
