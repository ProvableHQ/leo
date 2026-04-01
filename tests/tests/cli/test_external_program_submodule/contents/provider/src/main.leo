// Provider program: exposes a top-level const and entry functions that
// accept/return the Color struct defined in the colors submodule.
program provider.aleo {
    const CHANNELS: u32 = 3u32;

    // Build a Color from individual channel values.
    fn make_color(r: u32, g: u32, b: u32) -> colors::Color {
        return colors::Color { r: r, g: g, b: b };
    }

    // Sum all channels of a Color.
    fn sum_channels(c: colors::Color) -> u32 {
        return c.r + c.g + c.b;
    }

    @noupgrade
    constructor() {}
}
