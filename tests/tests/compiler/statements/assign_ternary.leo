program test.aleo {
    @noupgrade
    constructor() {}
}

fn main(y: u32) {
    let x: bool = true ? y : true;
}
