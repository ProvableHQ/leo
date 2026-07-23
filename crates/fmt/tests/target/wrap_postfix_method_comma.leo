program test.aleo {
    @noupgrade
    constructor() {}

    fn f() -> bool {
        return some_long_mapping_name_here
            .get_or_use(SomeKey { alpha: value_one, beta: value_two }, false);
    }
}
