program test.aleo {
    transition main() -> u32 {
        return foo::[10]();
    }

    inline foo::[P: u32]() -> u32 {
        let sum = 0u32;
        for i: u32 in 1..P {
            let arr = [i; i];

            for j: u32 in 0..i {
                sum += arr[j];
            }

            sum += bar::[i]([i; i]);
        }
        return sum;
    }

    inline bar::[K: u32](arr: [u32; K]) -> u32 {
        let sum = 0u32;
        for j: u32 in 0..K {
            sum += arr[j];
        }
        return sum;
    }
}
