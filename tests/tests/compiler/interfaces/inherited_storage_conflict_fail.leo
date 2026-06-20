export interface Storage1 {
    storage value: u64;
}

export interface Storage2 {
    storage value: u32;
}

export interface Combined: Storage1 + Storage2 {
}

program test.aleo: Combined {
    fn main() {}

    @noupgrade
    constructor() {}
}
