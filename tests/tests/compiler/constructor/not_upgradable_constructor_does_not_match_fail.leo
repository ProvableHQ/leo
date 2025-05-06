/*
upgrade = { "mode": "noupgrade" }
*/

program test.aleo {
    transition foo() {}
    
    async constructor() {
        assert_eq(self.edition, 0u16);
        assert_eq(self.edition, 0u16);
    }
}