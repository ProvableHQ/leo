program test.aleo {
    fn main(a: field) -> bool {
        let negOneField: field = -1field;
        let _negnegnegOneField: field = 0field - --1field;
        return negOneField + a == 0field;
    }

    @noupgrade
    constructor() {}
}
