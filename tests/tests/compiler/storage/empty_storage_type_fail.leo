program empty_storage_type.aleo {

    storage empty_array: [u8;0];
    storage vector_of_empty_array: [[u8; 0]];
	storage byte: u8;

    async transition prepare(a: [u32; 0]) ->  Future {
        return finalize(a);
    }


    async function finalize(a: [u32;0]) {
        empty_array = [];
        vector_of_empty_array = [];
		byte = 42u8;
    }

}
