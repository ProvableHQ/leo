program test.aleo {
    fn main() -> u32 {
        return foo::[3]();
    }
}
fn foo::[P: u32]() -> u32 {
    let sum = 0u32;
    for i: u32 in 0u32..P {
        let arr = [0u32; i];
        sum += arr[0u32];
    }
    return sum;
}
