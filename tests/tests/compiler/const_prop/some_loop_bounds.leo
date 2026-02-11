
const START: u32 = 0u32;
const STOP: u32 = 6u32;

final fn foo() {
    for i: u32 in START+1u32..STOP-2u32 {
        const SOME: u32 = i + 1u32;
        for j: u32 in SOME+1u32..STOP {
            m.set(i, j);
        }
    }
}

program test.aleo {
    mapping m: u32 => u32;
    fn main() -> Final {
        return final { foo(); };
    }
}
