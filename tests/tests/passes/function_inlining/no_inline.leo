fn add_inline(a: u32, b: u32) -> u32 {
    return a + b;
}

@no_inline
fn add_no_inline(a: u32, b: u32) -> u32 {
    return a + b;
}

program test.aleo {
    fn main(x: u32, y: u32) -> (u32, u32) {
        let result_inline: u32 = add_inline(x, y);
        let result_no_inline: u32 = add_no_inline(x, y);
        return (result_inline, result_no_inline);
    }
}
