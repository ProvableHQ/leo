// Test: Deeply transitive interface inheritance cycle (should fail)

export interface A: B {
    fn a() -> u64;
}

export interface B: C {
    fn b() -> u64;
}

export interface C: D {
    fn c() -> u64;
}

export interface D: A {
    fn d() -> u64;
}

program test.aleo {
    fn main() -> u64 {
        return 0u64;
    }

    @noupgrade
    constructor() {}
}
