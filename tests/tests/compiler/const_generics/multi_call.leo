program test.aleo {
    fn main() -> u32 {
        return bar::[5u32, 1u32]() + bar::[6u32, 2u32]();
    }
}

fn bar::[N: u32, M: u32]() -> u32 {
    return N + M + foo1() + foo2();
}

fn foo1() -> u32 { return 1000u32; }
fn foo2() -> u32 { return 10u32; }
