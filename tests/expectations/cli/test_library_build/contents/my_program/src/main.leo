program my_program.aleo {
    fn main() -> u32 {
        return my_lib::MAGIC + my_lib::DOUBLE_MAGIC;
    }

    @noupgrade
    constructor() {}
}
