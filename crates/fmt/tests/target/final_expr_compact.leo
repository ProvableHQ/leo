program test.aleo {
    fn simple(value: u64) -> Final {
        return final { finalize(value); };
    }

    fn tuple(value: u64) -> (u64, Final) {
        return (
            value,
            final {
                finalize(value);
            },
        );
    }

    fn commented(value: u64) -> Final {
        return final { // keeps comment attached
            finalize(value);
        };
    }
}

final fn finalize(value: u64) {
    return;
}
