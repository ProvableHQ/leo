// A query reading a storage variable and a vector. These compile to mappings
// under the hood, so the query needs the right access checks to pass.
program test.aleo {
    storage owner: address;
    storage entries: [u64];

    fn dummy(x: u32) -> u32 { return x; }

    query fn current_owner() -> address {
        return owner.unwrap_or(aleo1qnr4dkkvkgfqph0vzc3y6z2eu975wnpz2925ntjccd5cfqxtyu8s7pyjh9);
    }

    query fn entry_count() -> u32 {
        return entries.len();
    }

    query fn entry_at(idx: u32) -> u64 {
        return entries.get(idx).unwrap_or(0u64);
    }

    @noupgrade
    constructor() {}
}
