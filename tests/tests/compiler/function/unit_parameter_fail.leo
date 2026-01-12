
struct S {
    x: (),
    y: u8,
}

program test.aleo {
    transition main(x: ()) {}

    transition other(x: [(); 2]) {}

    transition yet_another() -> [(); 2] {}

}
