program test.aleo {
    fn main(input: u32) -> u32 {
        let var1 = wrapper(input);
        return var1;
    }
}

fn helper(x: u32) -> u32 {
    let var3 = 1u32;
    let var4 = x + var3;
    return var4;
}

fn wrapper(y: u32) -> u32 {
    let var6 = helper(y);
    let var7 = 2u32;
    let var8 = var6 * var7;
    return var8;
}
