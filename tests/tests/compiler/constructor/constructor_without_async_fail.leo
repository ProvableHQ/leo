program test.aleo {
    transition foo() {}
    
    constructor() {
        assert_eq(Program::edition("test.aleo"), 0u16);
        assert_eq(self.edition, 0u16);
    }
}