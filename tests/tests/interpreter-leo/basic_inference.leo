struct S {
    x: u32,
    y: u32,
}

program test.aleo {
    record R {
        owner: address,
        v: u64
    }

    fn main() -> (S, S, u64, u32, u32, bool, u32, u32) {
        return run(0, 1, [5, 6, 7, 8]);
    }
}

fn run(x: u32, y: u32, arr: [u32; 4]) 
    -> (S, S, u64, u32, u32, bool, u32, u32) {
    let s1: S = S { x, y };
    let s2: S = s1;
    let s3 = s1;
    //s3.y = 42u32;
    let t = (s2, s3, 42u64);
    let (x_plus_1, y_plus_2) = (x + 1u32, y + 2u32);
    let b = true;
    let call_result = foo(x_plus_1);
    let array_access = arr[3u8];
    return (t.0, t.1, t.2, x_plus_1, y_plus_2, b, call_result, array_access);
}

fn foo(x: u32) -> u32 {
    return x * x;
}
