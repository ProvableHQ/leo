program my_program.aleo {
    // Uses calc_lib, which internally calls base_lib.
    fn via_calc(x: u32) -> u32 {
        return calc_lib::quadruple(x);
    }

    // Uses base_lib directly so both libraries appear as stubs of this program.
    fn via_base(x: u32) -> u32 {
        return base_lib::double(x);
    }

    // Uses the generic base_lib::multiply with two different const arguments so that
    // monomorphization must instantiate multiply::[3u32] and multiply::[5u32] for a
    // FromLeo stub whose inner stubs field is empty.
    fn via_triple(x: u32) -> u32 {
        return base_lib::multiply::[3u32](x);
    }

    fn via_fivefold(x: u32) -> u32 {
        return base_lib::multiply::[5u32](x);
    }

    @noupgrade
    constructor() {}
}
