import provider.aleo;

program consumer.aleo {
    // Struct and const from the submodule.
    fn make_white() -> provider.aleo::colors::Color {
        return provider.aleo::colors::Color {
            r: provider.aleo::colors::MAX_CH,
            g: provider.aleo::colors::MAX_CH,
            b: provider.aleo::colors::MAX_CH,
        };
    }

    // Top-level entry function from the provider.
    fn mix(a: provider.aleo::colors::Color, b: provider.aleo::colors::Color) -> provider.aleo::colors::Color {
        return provider.aleo::mix_colors(a, b);
    }

    // Submodule helper called directly — inlined into consumer's bytecode.
    fn average(a: provider.aleo::colors::Color, b: provider.aleo::colors::Color) -> provider.aleo::colors::Color {
        return provider.aleo::colors::blend(a, b);
    }

    @noupgrade
    constructor() {}
}
