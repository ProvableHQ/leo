program constr.aleo {
    fn main() {}

    @custom
    constructor() {
        let x = std::ctx::program_owner();
    }
}
