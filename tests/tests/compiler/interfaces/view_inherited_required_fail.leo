// FAIL: a child interface inherits a `view fn` from a parent. The implementing
// program omits the parent's view. The inherited-prototype path through
// `flatten_interface` must surface view fns the same way it surfaces plain fns.
interface IBase {
    view fn total_supply() -> u128;
}

interface IExtended: IBase {
    fn poke() -> Final;
}

program scratch.aleo: IExtended {
    storage total_supply: u128;

    @noupgrade
    constructor() {}

    fn poke() -> Final {
        return final { };
    }
}
