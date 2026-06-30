program test.aleo {
    @noupgrade
    constructor() {}
}

export struct One {
    two: Two,
}

export struct Two {
    three: Three,
    four: Four,
}

export struct Three {
    one: One,
}

export struct Four {
    one: One,
}
