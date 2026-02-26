program test.aleo {
    fn main() -> u32 {
        return foo::[10]();
    }
}

fn foo::[P: u32]() -> u32 {
    let sum = 0u32;
    for i: u32 in 1..P {
        let arr = [0u32; i];
        for j: u32 in 0..i {
            arr[j] = i*i;
        }

        for j: u32 in 0..i {
            sum += arr[j];
        }
    }
    return sum;
}
