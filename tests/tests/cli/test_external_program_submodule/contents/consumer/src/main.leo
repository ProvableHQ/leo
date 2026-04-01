import provider.aleo;

program consumer.aleo {
    // Struct from submodule in type annotation and return position.
    fn identity(c: provider.aleo::colors::Color) -> provider.aleo::colors::Color {
        return c;
    }

    // Struct literal using submodule type; submodule const for channel cap.
    fn make_white() -> provider.aleo::colors::Color {
        return provider.aleo::colors::Color {
            r: provider.aleo::colors::MAX_CH,
            g: provider.aleo::colors::MAX_CH,
            b: provider.aleo::colors::MAX_CH,
        };
    }

    // Call a top-level entry fn from provider that accepts submodule types.
    fn sum_of_white() -> u32 {
        let white: provider.aleo::colors::Color = provider.aleo::colors::Color {
            r: provider.aleo::colors::MAX_CH,
            g: provider.aleo::colors::MAX_CH,
            b: provider.aleo::colors::MAX_CH,
        };
        return provider.aleo::sum_channels(white);
    }

    // Use provider's top-level constant.
    fn channel_count() -> u32 {
        return provider.aleo::CHANNELS;
    }

    @noupgrade
    constructor() {}
}
