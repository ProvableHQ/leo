import lib.aleo;

program main.aleo {
    @noupgrade
    constructor() {}

    fn quad(public x: u32) -> u32 {
        return lib.aleo::quadruple(x);
    }
}
