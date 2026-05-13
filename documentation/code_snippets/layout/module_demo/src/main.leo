program module_demo.aleo {
    fn use_module() -> u32 {
        return mymod::X;
    }

    @noupgrade
    constructor() {}
}
