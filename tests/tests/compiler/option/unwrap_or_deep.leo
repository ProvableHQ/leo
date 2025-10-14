program unwrap_or_deep.aleo {

    struct Inner {
        val: u8?,
    }

    struct Wrapper {
        arr: [Inner?; 1]?,
    }

    struct Container {
        wrappers: [Wrapper?; 1]?,
    }

    struct Top {
        container: Container?,
    }

    transition unwrap_or_deep() -> u8 {
        // Top structure with missing values deep inside
        let top: Top = Top {
            container: Container {
                wrappers: [Wrapper {
                    arr: [Inner { val: none }]
                }]
            }
        };

        // Fallback value for innermost level
        let fallback_inner = Inner { val: 55u8 };

        // Chain of unwrap_or with 1 fallback per level
        let val = top
            .container.unwrap_or(Container {
                wrappers: [Wrapper {
                    arr: [fallback_inner]
                }]
            })
            .wrappers.unwrap_or([Wrapper {
                arr: [fallback_inner]
            }])[0]
            .unwrap_or(Wrapper {
                arr: [fallback_inner]
            })
            .arr.unwrap_or([fallback_inner])[0]
            .unwrap_or(fallback_inner)
            .val.unwrap_or(99u8);

        return val;
    }
}
