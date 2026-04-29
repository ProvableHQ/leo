program credits.aleo {
    fn ping() {}

    @noupgrade
    constructor() {}
}

// --- Next Program --- //
program my_library.aleo {
    fn ping() {}

    @noupgrade
    constructor() {}
}

// --- Next Program --- //
// ANCHOR: file
import credits.aleo;
import my_library.aleo;

program my_program.aleo {
    // ...

    fn ping() {}

    @noupgrade
    constructor() {}
}
// ANCHOR_END: file
