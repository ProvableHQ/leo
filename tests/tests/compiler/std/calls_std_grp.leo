program main.aleo {
    fn exercise() {
        let rp1 = std::grp::generator();
        assert_eq(rp1, rp1);
        let rp2 = std::grp::aleo_generator();
        assert_eq(rp2, rp2);
        let rp3 = std::grp::aleo_generator_powers();
        assert_eq(rp3, rp3);
        let rp4 = std::grp::to_x_coordinate(0group);
        assert_eq(rp4, rp4);
        let rp5 = std::grp::to_y_coordinate(0group);
        assert_eq(rp5, rp5);
        return;
    }

    @noupgrade
    constructor() {}
}
