program test.aleo {
    transition foo() {}
    
    async constructor() {
        assert_eq(Program::program_owner("test.aleo"), aleo1rhgdu77hgyqd3xjj8ucu3jj9r2krwz6mnzyd80gncr5fxcwlh5rsvzp9px);
        assert_eq(self.program_owner, aleo1rhgdu77hgyqd3xjj8ucu3jj9r2krwz6mnzyd80gncr5fxcwlh5rsvzp9px);
    }
}