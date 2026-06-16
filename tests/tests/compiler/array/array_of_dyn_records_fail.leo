program test.aleo {
    fn test(
        tokens: [dyn record; 2],
    ){
    }

    fn test_2(
        token_1: dyn record,
        token_2: dyn record,
    ) -> [dyn record; 2] {
        let x: [dyn record; 2] = [token_1, token_2];
        return x;
    }

    @noupgrade
    constructor() {}
}
