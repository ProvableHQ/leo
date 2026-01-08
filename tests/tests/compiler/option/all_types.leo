program all_types.aleo {

    // --- Shared Structs ---
    struct OptStruct {
        val: u16?,
    }

    struct MyStruct {
        x: u8,
    }

    struct Foo {
        x: u8,
    }

    struct Inner {
        val: u8?,
    }

    struct Wrapper {
        arr: [Inner?; 2]?,
    }

    struct Container {
        wrappers: [Wrapper?; 2]?,
    }

    struct Top {
        container: Container?,
    }

    // --- Section 1: Basic Optionals and Inference ---
    transition optional_basic_primitives() -> u32 {
        let a: u32? = 10u32;
        let b: u32? = none;
        let c: u32 = a.unwrap();
        let d: u32 = b.unwrap_or(42u32);
        return c + d; // 10 + 42 = 52
    }

    // --- Section 2: Arrays of Optionals ---
    transition array_of_optionals() -> u8 {
        let arr: [u8?; 3] = [1u8, none, 3u8];
        let sum = arr[0].unwrap_or(10u8)
                  + arr[1].unwrap_or(20u8)
                  + arr[2].unwrap_or(30u8);
        return sum; // 1 + 20 + 3 = 24
    }

    // --- Section 3: Optional Arrays ---
    transition optional_array() -> u8 {
        let arr_opt: [u8?; 2]? = [1u8, none];
        let first = arr_opt.unwrap()[0].unwrap_or(10u8);
        return first; // 1
    }

    // --- Section 4: Structs with Optional Fields ---
    transition optional_struct_field() -> u16 {
        let s = OptStruct { val: none };
        return s.val.unwrap_or(123u16); // 123
    }

    // --- Section 5: Optional Structs ---
    transition optional_struct() -> u8 {
        let s: MyStruct? = MyStruct { x: 5u8 };
        return s.unwrap().x; // 5
    }

    // --- Section 6: Arrays of Optional Structs ---
    transition array_of_optional_structs() -> u8 {
        let arr: [Foo?; 2] = [Foo { x: 10u8 }, none];
        return arr[0].unwrap().x; // 10
    }

    // --- Section 7: Nested Optionals in Structs & Arrays ---
    transition nested_optional_structs() -> u8 {
        let inner = Inner { val: 10u8 };
        let wrapper = Wrapper { arr: [inner, none] };
        let container = Container { wrappers: [wrapper, none] };
        let top = Top { container };

        return top.container.unwrap()
            .wrappers.unwrap()[0].unwrap()
            .arr.unwrap()[0].unwrap()
            .val.unwrap(); // 10
    }

    // --- Section 8: Tuples with Optional Elements ---
    transition tuples_with_optional_elements() -> u32 {
        let t1: (u32, u32?) = (100u32, 200u32);
        let t2: (u32, u32?) = (300u32, none);

        let sum = t1.0 + t1.1.unwrap_or(0u32)
                + t2.0 + t2.1.unwrap_or(50u32);
        return sum; // 100 + 200 + 300 + 50 = 650
    }

    // --- Section 9: Optionals with group, field, scalar ---
    transition optional_misc_primitive_types() -> (group, field, scalar) {
        let gr: group? = 0group;
        let fl: field? = 1field;
        let sc: scalar? = 2scalar;

        return (
            gr.unwrap(),
            fl.unwrap_or(2field),
            sc.unwrap(),
        );
    }

    // --- Section 10: Optionals with Address ---
    struct AddrStruct { a: address, b: address? }
    struct InnerAddr { val: address? }
    struct WrapperAddr { inner: InnerAddr }
    transition optional_address_allowed() -> (address, address, address, address) {
        // Case 1: Direct optional address
        let a: address? = none;
        let b: address? = aleo10qerras5799u6k7rjtc9y3hcwxuykr45qra7x7dp6jgnc0923czqm0lgta;
        let val1 = b.unwrap_or(a.unwrap_or(aleo10qerras5799u6k7rjtc9y3hcwxuykr45qra7x7dp6jgnc0923czqm0lgta));

        // Case 2: Struct containing address and optional address
        let s = AddrStruct { 
            a: aleo10qerras5799u6k7rjtc9y3hcwxuykr45qra7x7dp6jgnc0923czqm0lgta, 
            b: none 
        };
        let val2 = s.b.unwrap_or(s.a);

        // Case 3: Optional struct containing address
        let s_opt: AddrStruct? = AddrStruct { 
            a: aleo10qerras5799u6k7rjtc9y3hcwxuykr45qra7x7dp6jgnc0923czqm0lgta, 
            b: none 
        };
        let val3 = s_opt.unwrap().a;

        // Case 4: Nested struct with optional address field

        let wrapper = WrapperAddr { inner: InnerAddr { val: aleo10qerras5799u6k7rjtc9y3hcwxuykr45qra7x7dp6jgnc0923czqm0lgta } };
        let val4 = wrapper.inner.val.unwrap();

        // Just return one of the addresses to satisfy return type
        return (val1, val2, val3, val4);
    }

    // --- Section 11: Optionals with Signature ---
    struct SigStruct { a: signature, b: signature? }
    struct InnerSig { val: signature? }
    struct WrapperSig { inner: InnerSig }
    transition optional_signature() -> (signature, signature, signature, signature)
    {
        let sig: signature =
            sign195m229jvzr0wmnshj6f8gwplhkrkhjumgjmad553r997u7pjfgpfz4j2w0c9lp53mcqqdsmut2g3a2zuvgst85w38hv273mwjec3sqjsv9w6uglcy58gjh7x3l55z68zsf24kx7a73ctp8x8klhuw7l2p4s3aq8um5jp304js7qcnwdqj56q5r5088tyvxsgektun0rnmvtsuxpe6sj;

        // Case 1: Direct optional signature
        let a: signature? = none;
        let b: signature? = sig;
        let val1 = b.unwrap_or(a.unwrap_or(sig));

        // Case 2: Struct containing signature + optional signature
        let s = SigStruct {
            a: sig,
            b: none
        };
        let val2 = s.b.unwrap_or(s.a);

        // Case 3: Optional struct containing signature
        let s_opt: SigStruct? = SigStruct {
            a: sig,
            b: none
        };
        let val3 = s_opt.unwrap().a;

        // Case 4: Nested struct with optional signature
        let wrapper = WrapperSig {
            inner: InnerSig { val: sig }
        };
        let val4 = wrapper.inner.val.unwrap();

        return (val1, val2, val3, val4);
    }
}
