program empty_storage_type.aleo {

    storage empty_array: [u8;0];
    storage vector_of_empty_array: [[u8; 0]];
	storage byte: u8;

    fn prepare(a: [u32; 0]) ->  Final {
        return final { finalize(a); };
    }
}

final fn finalize(a: [u32;0]) {
    empty_array = [];
    vector_of_empty_array = [];
	byte = 42u8;
}

