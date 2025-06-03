/*
upgrade = { "mode": "admin", "address": "aleo1rhgdu77hgyqd3xjj8ucu3jj9r2krwz6mnzyd80gncr5fxcwlh5rsvzpx" }
*/

program test.aleo {
    transition foo() {}
    
    async constructor() {
        assert_eq(self.program_owner, aleo1rhgdu77hgyqd3xjj8ucu3jj9r2krwz6mnzyd80gncr5fxcwlh5rsvzp9px);
    }
}
