program st.aleo {
    storage s_field: field;
    storage s_vec: [u32];
    mapping s_map: u32 => u32;

    async transition assign_optional() -> Future {
        return async {
            let opt_val: field? = 5field;
            s_field = opt_val; // ❌ illegal
        };
    }

    async transition assign_to_vector() -> Future {
        return async {
            s_vec = [1u32, 2u32]; // ❌ cannot assign whole vector directly
            s_vec = none;         // ❌ cannot assign none to a storage vector
        };
    }

    async transition assign_vector_to_var() -> Future {
        return async {
            let local_vec1 = s_vec;       // ❌ cannot move storage vector
            let local_vec2 = [0u32; 2];
            local_vec2 = s_vec;           // ❌ cannot copy storage vector
        };
    }

    async transition wrong_vector_ops() -> Future {
        return async {
            let x: field = 5field;
            x.push(3field);        // ❌ not a vector
            x.pop();               // ❌ not a vector
            x.len();               // ❌ not a vector
            x.swap_remove(0u32);   // ❌ not a vector
            x.set(0u32, 1field);   // ❌ not a vector
            x.get(0u32);           // ❌ not a vector
        };
    }

    async transition wrong_vector_ops_on_storage() -> Future {
        return async {
            s_field.push(10field);     // ❌ cannot push to non-vector storage
            s_map.push(5u32);          // ❌ cannot push to mapping
        };
    }

    async transition wrong_set_get() -> Future {
        return async {
            s_field.set(0u32, 10field);  // ❌ not a vector or mapping
            s_field.get(0u32);           // ❌ not a vector or mapping
        };
    }
}
