program my_program.aleo {
    fn main() -> u32 {
        return bad_lib::BROKEN;
    }

    @noupgrade
    constructor() {}
}
