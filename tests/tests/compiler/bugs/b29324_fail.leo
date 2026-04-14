// Regression test for https://github.com/ProvableHQ/leo/issues/29324
// The compiler should emit a diagnostic error, not panic, when `final { }` is
// used as a `const` initializer at program scope.
program test.aleo {
    const F: Final = final { };

    @noupgrade
    constructor() {}
}
