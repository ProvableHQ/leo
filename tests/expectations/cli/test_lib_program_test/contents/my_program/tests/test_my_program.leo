import my_program.aleo;

program test_my_program.aleo {
    // via_calc(3) = calc_lib::quadruple(3) = base_lib::double(base_lib::double(3)) = 12.
    @test
    fn test_via_calc() {
        let result: u32 = my_program.aleo::via_calc(3u32);
        assert_eq(result, 12u32);
    }

    // via_base(5) = base_lib::double(5) = 10.
    @test
    fn test_via_base() {
        let result: u32 = my_program.aleo::via_base(5u32);
        assert_eq(result, 10u32);
    }

    // via_triple(4) = base_lib::multiply::[3u32](4) = 12.
    @test
    fn test_via_triple() {
        let result: u32 = my_program.aleo::via_triple(4u32);
        assert_eq(result, 12u32);
    }

    // via_fivefold(4) = base_lib::multiply::[5u32](4) = 20.
    @test
    fn test_via_fivefold() {
        let result: u32 = my_program.aleo::via_fivefold(4u32);
        assert_eq(result, 20u32);
    }

    @noupgrade
    constructor() {}
}
