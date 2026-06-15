program test.aleo {
    fn main(a: scalar, b: group, c: scalar) -> bool {
        let d: group = 1817767092074430972953743941103352519057913259183777531581123188265134806220group
            * a;
        let _e: group = a
            * 1817767092074430972953743941103352519057913259183777531581123188265134806220group;
        let _f: group = b * a;
        let g: group = a * b;

        return a * g == d;
    }

    @noupgrade
    constructor() {}
}
