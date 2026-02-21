
struct S {
    x: (),
    y: u8,
}

program test.aleo {
    fn main(x: ()) {}

    fn other(x: [(); 2]) {}

    fn yet_another() -> [(); 2] {}

}
