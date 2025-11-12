program empty_type_mappings.aleo {

    mapping foo: u64 => [u32; 0];
    mapping bar: [u32; 0] => u64;

    async transition prepare(a: [u32; 0]) ->  Future {
        return finalize(a);
    }


    async function finalize(a: [u32;0]) {
        Mapping::set(foo, 0, []);
        Mapping::set(bar, [], 0);
    }

}
