
program test.aleo {
    fn main() -> i32 {
        const A: i32 = 1i32;

        const B: i32 = 12i32;

        const C: (i32, i32) = (1i32 + A, A * B);

        return C.1;
    }
}
