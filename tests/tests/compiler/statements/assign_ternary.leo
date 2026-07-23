program test.aleo {
    @noupgrade
    constructor() {}
}

export fn main(y: u32) {
    let x: bool = true ? y : true;
}
