// The `_`-prefixed annotations are compiler-internal and only recognised inside the standard
// library. User code that spells them should get the same "unknown annotation" error as any
// unknown identifier annotation.
program test.aleo {
    @_caller_annotation
    fn a() -> address { return 0field as address; }

    @_program_id_arg
    fn b() -> u16 { return 0u16; }

    @_callable_function_arg
    fn c() -> u16 { return 0u16; }

    @_onchain_context
    fn d() -> u16 { return 0u16; }

    @noupgrade
    constructor() {}
}
