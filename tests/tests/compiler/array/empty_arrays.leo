program empty_arrays.aleo {
    fn main(a: [u32; 0], c: [u32; 0]) ->  ([u32;0], u32, [u64;0], u64) {
        let val: [u32;0u64] = [];
        let val_64: [u64;0] = [];
        val = id(val);
        return (foo(0, val), sum::[0](val), val_64, sum_64::[0](val_64));
    }
}

fn foo(a: u32, b:[u32; 0]) -> [u32;0] {
    let c = b;
    let d = c;
    return d;
}

fn id(a: [u32;0]) -> [u32;0] {
    return a;
} 

fn sum::[N: u32](arr: [u32; N]) -> u32 {
    let total = 0u32;
    for i in 0u32..N {
        total += arr[i];
    }
    return total;
}
fn sum_64::[N: u64](arr: [u64; N]) -> u64 {
    let total = 0u64;
    for i in 0u64..N {
        total += arr[i];
    }
    return total;
}
