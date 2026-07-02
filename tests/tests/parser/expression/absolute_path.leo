program test.aleo {
    fn main() -> u32 {
        let a = ::foo::X;
        let b = ::foo::bar::f(1u32);
        let s = ::foo::S { x: 1u32 };
        let g = ::foo::Gen::[2] { vals: [1u32, 2u32] };
        return a + b + s.x + g.vals[0u32];
    }
}
