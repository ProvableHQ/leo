// FAIL: interfaces may only declare `fn` or `view fn` prototypes. `final fn` is not a
// valid prototype variant and must be rejected by the parser.
interface IBad {
    final fn step();
}

program scratch.aleo: IBad {
    @noupgrade
    constructor() {}

    fn step() {}
}
