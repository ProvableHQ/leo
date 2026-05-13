program provider.aleo {
    fn sum_channels(c: colors::Color) -> u32 {
        return c.r + c.g + c.b;
    }

    fn mix_colors(a: colors::Color, b: colors::Color) -> colors::Color {
        return colors::blend(a, b);
    }

    @noupgrade
    constructor() {}
}
